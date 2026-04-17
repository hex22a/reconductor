use anyhow::Ok;
use lapin::{BasicProperties, Channel, options::BasicPublishOptions, types::ShortString};
use serde_json::Value;

pub trait MqProvider {
    async fn publish(&self, channel: ShortString, message: Value) -> anyhow::Result<()>;
}

pub struct RabbitMqProvider {
    pub channel: Channel,
}

impl MqProvider for RabbitMqProvider {
    async fn publish(&self, channel: ShortString, message: Value) -> anyhow::Result<()> {
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
