use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

pub struct RabbitMqPublisher {
    pub channel: Channel,
}

pub trait ScanPublisher {
    async fn publish(&self, scan_id: Uuid, target: &IpNetwork) -> anyhow::Result<()>;
}

impl ScanPublisher for RabbitMqPublisher {
    async fn publish(&self, scan_id: Uuid, target: &IpNetwork) -> anyhow::Result<()> {
        let message = serde_json::json!({ "id": scan_id, "target": target });
        self.channel
            .basic_publish(
                "".into(),
                "scans".into(),
                BasicPublishOptions::default(),
                serde_json::to_vec(&message)?.as_slice(),
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?;
        Ok(())
    }
}
