use lapin::{BasicProperties, Channel, options::BasicPublishOptions, types::ShortString};
use serde_json::Value;

use crate::infra::message_queue::error::MqError;

pub mod error;
pub mod publisher;

pub trait MqProvider {
    fn publish(
        &self,
        channel: ShortString,
        message: Value,
    ) -> impl Future<Output = Result<(), MqError>> + Send;
}

pub struct RabbitMqProvider {
    channel: Channel,
}

impl RabbitMqProvider {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl MqProvider for RabbitMqProvider {
    async fn publish(&self, channel: ShortString, message: Value) -> Result<(), MqError> {
        self.channel
            .basic_publish(
                "".into(),
                channel,
                BasicPublishOptions::default(),
                serde_json::to_vec(&message)?.as_slice(),
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?;
        Ok(())
    }
}
