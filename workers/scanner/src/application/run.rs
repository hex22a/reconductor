use crate::{
    application::{error::AppError, parser},
    domain::{result::Host, scan_message::ScanMessage},
    features::{
        scan::{model::ScanStatus, update::UpdateScanFeature},
        scan_result::add::AddScanResultFeature,
    },
    infra::{message_queue::consumer::Consumer, nmap::ScanRunner},
};
use futures_lite::StreamExt;
use tracing::{error, info};
use uuid::Uuid;

pub trait Runner {
    fn run(&self) -> impl Future<Output = Result<(), AppError>>;
    fn process(&self, scan_id: Uuid, target: &str) -> impl Future<Output = Result<(), AppError>>;
}

pub struct ApplicationRunner<
    C: Consumer,
    S: ScanRunner,
    U: UpdateScanFeature,
    A: AddScanResultFeature,
> {
    consumer: C,
    scan_runner: S,
    update_scan_feature: U,
    add_scan_result_feature: A,
}

impl<C, S, U, A> ApplicationRunner<C, S, U, A>
where
    C: Consumer,
    S: ScanRunner,
    U: UpdateScanFeature,
    A: AddScanResultFeature,
{
    pub fn new(
        consumer: C,
        scan_runner: S,
        update_scan_feature: U,
        add_scan_result_feature: A,
    ) -> Self {
        Self {
            consumer,
            scan_runner,
            update_scan_feature,
            add_scan_result_feature,
        }
    }
}

impl<C, S, U, A> Runner for ApplicationRunner<C, S, U, A>
where
    C: Consumer,
    S: ScanRunner,
    U: UpdateScanFeature,
    A: AddScanResultFeature,
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

            match self.process(msg.id, &msg.target).await {
                Ok(_) => {
                    info!("Scan {} completed", msg.id);
                    self.consumer.ack(&delivery).await?;
                }
                Err(e) => {
                    error!("Scan {} failed: {}", msg.id, e);
                    self.consumer.nack(&delivery, true).await?;
                }
            }
        }

        Ok(())
    }

    async fn process(&self, scan_id: Uuid, target: &str) -> Result<(), AppError> {
        self.update_scan_feature
            .update_scan_status(scan_id, ScanStatus::InProgress)
            .await?;

        let xml = S::run(target).await?;
        let result = parser::parse(&xml)?;

        let hosts: Vec<Host> = result
            .hosts
            .into_iter()
            .filter(|h| h.status.state == "up")
            .collect();

        self.add_scan_result_feature
            .add_scan_results(scan_id, hosts)
            .await?;

        self.update_scan_feature
            .update_scan_status(scan_id, ScanStatus::Done)
            .await?;
        Ok(())
    }
}
