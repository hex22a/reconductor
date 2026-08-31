use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::infra::message_queue::provider::MqProvider;

pub trait Publisher {
    fn publish_scan(
        &self,
        scan_id: Uuid,
        target: &IpNetwork,
    ) -> impl Future<Output = anyhow::Result<()>>;
}

pub struct MqPublisher<T: MqProvider> {
    pub provider: T,
}

impl<T: MqProvider> Publisher for MqPublisher<T> {
    async fn publish_scan(&self, scan_id: Uuid, target: &IpNetwork) -> anyhow::Result<()> {
        let message = serde_json::json!({ "id": scan_id, "target": target });
        self.provider.publish("scans".into(), message).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };

    use anyhow::Ok;
    use lapin::types::ShortString;
    use serde_json::Value;
    use sqlx::types::ipnetwork::{IpNetwork, Ipv4Network};
    use uuid::Uuid;

    use crate::infra::message_queue::provider::MqProvider;

    struct MockMqProvider {
        publish_calls: Arc<Mutex<Vec<(ShortString, Value)>>>,
    }

    impl MqProvider for MockMqProvider {
        async fn publish(&self, channel: ShortString, message: Value) -> anyhow::Result<()> {
            self.publish_calls.lock().unwrap().push((channel, message));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_publush_scan() {
        // Arrange
        let expected_scan_id: Uuid = Uuid::now_v7();
        let expected_target: IpNetwork =
            IpNetwork::V4(Ipv4Network::from_str("192.168.0.0/16").unwrap());
        let expected_channel: ShortString = "scans".into();
        let expected_message: Value =
            serde_json::json!({ "id": expected_scan_id, "target": expected_target });
        let publish_calls: Arc<Mutex<Vec<(ShortString, Value)>>> = Arc::new(Mutex::new(vec![]));
        let mock_mq_provider = MockMqProvider {
            publish_calls: publish_calls.clone(),
        };
        let publisher = MqPublisher {
            provider: mock_mq_provider,
        };

        // Act
        publisher
            .publish_scan(expected_scan_id, &expected_target)
            .await
            .unwrap();

        // Assert
        assert_eq!(publish_calls.lock().unwrap().len(), 1);
        assert_eq!(
            publish_calls.lock().unwrap()[0],
            (expected_channel, expected_message)
        )
    }
}
