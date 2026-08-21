use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::features::host::{
    dto::HostDto, error::HostError, model::HostEntity, repository::HostRepository,
};

pub trait GetHostFeature {
    fn get(
        &self,
        host_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<HostDto, HostError>> + Send + '_>>;
}

pub struct GetHost<R: HostRepository> {
    host_repository: Arc<R>,
}

impl<R: HostRepository> GetHost<R> {
    pub fn new(host_repository: Arc<R>) -> Self {
        Self { host_repository }
    }
}

impl<R> GetHostFeature for GetHost<R>
where
    R: HostRepository + Send + Sync,
{
    fn get(
        &self,
        host_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<HostDto, HostError>> + Send + '_>> {
        Box::pin(async move {
            let HostEntity {
                id,
                ip,
                mac,
                vendor,
                hostname,
                os_match,
                os_accuracy,
                ..
            } = self.host_repository.get_host(&host_id).await?;
            Ok(HostDto {
                id,
                ip: ip.ip(),
                mac,
                vendor,
                hostname,
                os_match,
                os_accuracy: os_accuracy.map(|a| format!("{}", a)),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use sqlx::types::ipnetwork::IpNetwork;

    use crate::features::host::model::HostEntity;

    use super::*;

    struct MockHostRepository {
        error: Mutex<Option<sqlx::Error>>,
        host_entity: HostEntity,
    }

    impl HostRepository for MockHostRepository {
        async fn get_host(&self, _: &Uuid) -> Result<HostEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.host_entity.clone()),
            }
        }

        async fn list_hosts(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<HostEntity>, sqlx::Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_get_host() {
        // Arrange
        let expected_host_id = Uuid::now_v7();
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host = HostEntity {
            id: expected_host_id,
            scan_run_id: expected_scan_run_id,
            ip: expected_ip,
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let expected_host_dto = HostDto {
            id: expected_host_id,
            ip: expected_ip.ip(),
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(None),
            host_entity: expected_host,
        };
        let feature = GetHost::new(Arc::new(mock_host_repository));

        // Act
        let actual_host_dto = feature.get(expected_host_id).await.unwrap();

        // Assert
        assert_eq!(actual_host_dto, expected_host_dto);
    }

    #[tokio::test]
    async fn test_get_host_not_found() {
        // Arrange
        let expected_host_id = Uuid::now_v7();
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host = HostEntity {
            id: expected_host_id,
            scan_run_id: expected_scan_run_id,
            ip: expected_ip,
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            host_entity: expected_host,
        };
        let feature = GetHost::new(Arc::new(mock_host_repository));

        // Act
        let actual_result = feature.get(expected_host_id).await;

        // Assert
        assert!(matches!(actual_result, Err(HostError::NotFound)));
    }
}
