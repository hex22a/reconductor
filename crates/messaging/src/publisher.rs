#[cfg(test)]
use mockall::automock;

use ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::{MqProvider, SCANS_QUEUE, error::MqError};

#[cfg_attr(test, automock)]
pub trait Publisher {
    fn publish_scan(
        &self,
        scan_id: &Uuid,
        target: &IpNetwork,
    ) -> impl Future<Output = Result<(), MqError>> + Send;
}

pub struct MqPublisher<T: MqProvider> {
    provider: T,
}

impl<T: MqProvider> MqPublisher<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T> Publisher for MqPublisher<T>
where
    T: MqProvider + Send + Sync,
{
    async fn publish_scan(&self, scan_id: &Uuid, target: &IpNetwork) -> Result<(), MqError> {
        let message = serde_json::json!({ "id": scan_id, "target": target });
        self.provider.publish(SCANS_QUEUE.into(), message).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::MockMqProvider;

    use super::*;

    use std::str::FromStr;

    use ipnetwork::Ipv4Network;
    use lapin::types::ShortString;
    use mockall::predicate::eq;
    use serde_json::Value;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_publish_scan() {
        // Arrange
        let expected_scan_id: Uuid = Uuid::now_v7();
        let expected_target: IpNetwork =
            IpNetwork::V4(Ipv4Network::from_str("192.168.0.0/16").unwrap());
        let expected_channel: ShortString = "scans".into();
        let expected_message: Value =
            serde_json::json!({ "id": expected_scan_id, "target": expected_target });

        let mut mock_mq_provider = MockMqProvider::new();
        mock_mq_provider
            .expect_publish()
            .with(eq(expected_channel), eq(expected_message))
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let publisher = MqPublisher {
            provider: mock_mq_provider,
        };

        // Act
        let actual_result = publisher
            .publish_scan(&expected_scan_id, &expected_target)
            .await;

        // Assert
        assert!(actual_result.is_ok());
    }
}
