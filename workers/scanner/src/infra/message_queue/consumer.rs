use lapin::message::Delivery;

use crate::infra::message_queue::{MqProvider, error::MqError};

pub trait Consumer {
    fn consume_scan(&self) -> impl Future<Output = Result<lapin::Consumer, MqError>> + Send;
    fn ack(&self, delivery: &Delivery) -> impl Future<Output = Result<(), MqError>> + Send;
    fn nack(
        &self,
        delivery: &Delivery,
        requeue: bool,
    ) -> impl Future<Output = Result<(), MqError>> + Send;
}

pub struct MqConsumer<T: MqProvider> {
    provider: T,
}

impl<T: MqProvider> MqConsumer<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T: MqProvider + Sync + Send> Consumer for MqConsumer<T> {
    async fn consume_scan(&self) -> Result<lapin::Consumer, MqError> {
        self.provider.consume().await
    }

    async fn ack(&self, delivery: &Delivery) -> Result<(), MqError> {
        T::ack(delivery).await
    }

    async fn nack(&self, delivery: &Delivery, requeue: bool) -> Result<(), MqError> {
        T::nack(delivery, requeue).await
    }
}
