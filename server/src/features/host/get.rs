use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::features::host::{
    dto::HostDto, error::HostError, model::HostEntity, repository::HostRepository,
};

#[cfg_attr(test, automock)]
pub trait GetHostFeature {
    fn get<'a>(
        &'a self,
        host_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<HostDto, HostError>> + Send + 'a>>;
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
    use sqlx::types::ipnetwork::IpNetwork;

    use crate::features::host::{model::HostEntity, repository::MockHostRepository};

    use super::*;

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

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository.expect_get_host().returning(move |_| {
            let host = expected_host.clone();
            Box::pin(async { Ok(host) })
        });

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

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository
            .expect_get_host()
            .returning(|_| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = GetHost::new(Arc::new(mock_host_repository));

        // Act
        let actual_result = feature.get(expected_host_id).await;

        // Assert
        assert!(matches!(actual_result, Err(HostError::NotFound)));
    }
}
