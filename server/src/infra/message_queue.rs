use lapin::{
    BasicProperties, Channel,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{FieldTable, ShortString},
};
use serde_json::Value;

use crate::{constants::SCANS_QUEUE, infra::message_queue::error::MqError};

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
    pub async fn build(channel: Channel) -> Result<Self, MqError> {
        channel
            .queue_declare(
                SCANS_QUEUE.into(),
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await
            .or(Err(MqError::BuildError));
        Ok(Self { channel })
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
