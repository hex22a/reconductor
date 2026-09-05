use lapin::{
    Channel, Consumer,
    message::Delivery,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
};

use crate::{
    constants::{CONSUMER_TAG, SCANS_QUEUE},
    infra::message_queue::error::MqError,
};

pub mod consumer;
pub mod error;

pub trait MqProvider {
    fn consume(&self) -> impl Future<Output = Result<Consumer, MqError>> + Send;
    fn ack(delivery: &Delivery) -> impl Future<Output = Result<(), MqError>> + Send;
    fn nack(delivery: &Delivery, requeue: bool)
    -> impl Future<Output = Result<(), MqError>> + Send;
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
            .or(Err(MqError::BuildError))?;

        channel.basic_qos(10, BasicQosOptions::default()).await?;
        Ok(Self { channel })
    }
}

impl MqProvider for RabbitMqProvider {
    async fn consume(&self) -> Result<Consumer, MqError> {
        self.channel
            .basic_consume(
                SCANS_QUEUE.into(),
                CONSUMER_TAG.into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .or(Err(MqError::ConsumeError))
    }

    async fn ack(delivery: &Delivery) -> Result<(), MqError> {
        delivery.ack(BasicAckOptions::default()).await?;
        Ok(())
    }

    async fn nack(delivery: &Delivery, requeue: bool) -> Result<(), MqError> {
        delivery
            .nack(BasicNackOptions {
                requeue,
                ..Default::default()
            })
            .await?;
        Ok(())
    }
}
