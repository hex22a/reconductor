use std::sync::Arc;

use crate::{
    application::{error::AppError, processor::Processor},
    domain::scan_message::ScanMessage,
    infra::message_queue::consumer::Consumer,
};
use futures_lite::StreamExt;
use lapin::message::Delivery;
use tracing::{error, info};

pub trait Runner {
    fn run(&self) -> impl Future<Output = Result<(), AppError>>;
    fn spawn_scan(&self, delivery: Delivery, msg: ScanMessage);
}

pub struct ApplicationRunner<P: Processor, C: Consumer> {
    processor: Arc<P>,
    consumer: Arc<C>,
}

impl<P, C> ApplicationRunner<P, C>
where
    P: Processor,
    C: Consumer,
{
    pub fn new(processor: Arc<P>, consumer: Arc<C>) -> Self {
        Self {
            processor,
            consumer,
        }
    }
}

impl<P, C> Runner for ApplicationRunner<P, C>
where
    P: Processor + Send + Sync + 'static,
    C: Consumer + Send + Sync + 'static,
{
    async fn run(&self) -> Result<(), AppError> {
        let mut consumer = self.consumer.consume_scan().await?;
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery?;

            let msg: ScanMessage = match serde_json::from_slice(&delivery.data) {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to deserialize message: {}", e);
                    self.consumer.nack(&delivery, false).await?;
                    continue;
                }
            };

            info!("Received scan job {} for target {}", msg.id, msg.target);

            self.spawn_scan(delivery, msg);
        }

        Ok(())
    }

    fn spawn_scan(&self, delivery: Delivery, msg: ScanMessage) {
        let consumer = Arc::clone(&self.consumer);
        let processor = Arc::clone(&self.processor);
        tokio::spawn(async move {
            match processor.process(msg.id, &msg.target).await {
                Ok(_) => {
                    info!("Scan {} completed", msg.id);
                    if let Err(e) = consumer.ack(&delivery).await {
                        error!("Failed to acknowledge {}", e)
                    };
                }
                Err(e) => {
                    error!("Scan {} failed: {}", msg.id, e);
                    if let Err(ne) = consumer.nack(&delivery, true).await {
                        error!("Failed to negative acknowledge {} on {}", ne, e)
                    };
                }
            }
        });
    }
}
