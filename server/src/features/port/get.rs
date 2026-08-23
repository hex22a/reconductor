use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::features::port::{
    dto::PortDto, error::PortError, model::PortEntity, repository::PortRepository,
};

pub trait GetPortFeature {
    fn get(
        &self,
        port_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<PortDto, PortError>> + Send + '_>>;
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
    use std::sync::Mutex;

    use crate::features::port::model::PortEntity;

    use super::*;

    struct MockPortRepository {
        error: Mutex<Option<sqlx::Error>>,
        port_entity: PortEntity,
    }

    impl PortRepository for MockPortRepository {
        async fn get_port(&self, _: &Uuid) -> Result<PortEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.port_entity.clone()),
            }
        }

        async fn list_ports(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<PortEntity>, sqlx::Error> {
            todo!()
        }
    }

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
        let mock_port_repository = MockPortRepository {
            error: Mutex::new(None),
            port_entity: expected_port,
        };
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
        };
        let mock_port_repository = MockPortRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            port_entity: expected_port,
        };
        let feature = GetPort::new(Arc::new(mock_port_repository));

        // Act
        let actual_result = feature.get(expected_port_id).await;

        // Assert
        assert!(matches!(actual_result, Err(PortError::NotFound)));
    }
}
