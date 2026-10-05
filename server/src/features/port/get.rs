use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::features::port::{
    dto::PortDto, error::PortError, model::PortEntity, repository::PortRepository,
};

#[cfg_attr(test, automock)]
pub trait GetPortFeature {
    fn get<'a>(
        &'a self,
        port_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<PortDto, PortError>> + Send + 'a>>;
}

pub struct GetPort<R: PortRepository> {
    port_repository: Arc<R>,
}

impl<R: PortRepository> GetPort<R> {
    pub fn new(port_repository: Arc<R>) -> Self {
        Self { port_repository }
    }
}

impl<R> GetPortFeature for GetPort<R>
where
    R: PortRepository + Send + Sync,
{
    fn get(
        &self,
        port_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<PortDto, PortError>> + Send + '_>> {
        Box::pin(async move {
            let PortEntity {
                id,
                port,
                protocol,
                state,
                service,
                product,
                version,
                ..
            } = self.port_repository.get_port(&port_id).await?;
            Ok(PortDto {
                id,
                port,
                protocol,
                state,
                service,
                product,
                version,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::features::port::{model::PortEntity, repository::MockPortRepository};

    use super::*;

    #[tokio::test]
    async fn test_get_port() {
        // Arrange
        let expected_port_id = Uuid::now_v7();
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port = PortEntity {
            id: expected_port_id,
            host_id: expected_host_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
            cpes: None,
        };
        let expected_port_dto = PortDto {
            id: expected_port_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
        };

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository.expect_get_port().returning(move |_| {
            let port = expected_port.clone();
            Box::pin(async { Ok(port) })
        });

        let feature = GetPort::new(Arc::new(mock_port_repository));

        // Act
        let actual_port_dto = feature.get(expected_port_id).await.unwrap();

        // Assert
        assert_eq!(actual_port_dto, expected_port_dto);
    }

    #[tokio::test]
    async fn test_get_port_not_found() {
        // Arrange
        let expected_port_id = Uuid::now_v7();
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port = PortEntity {
            id: expected_port_id,
            host_id: expected_host_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
            cpes: None,
        };

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository
            .expect_get_port()
            .returning(|_| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = GetPort::new(Arc::new(mock_port_repository));

        // Act
        let actual_result = feature.get(expected_port_id).await;

        // Assert
        assert!(matches!(actual_result, Err(PortError::NotFound)));
    }
}
