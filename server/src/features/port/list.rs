use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::{
    constants::PORTS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::port::{dto::PortDto, error::PortError, repository::PortRepository},
    transport::pagination::{Page, PageInfo},
};

#[cfg_attr(test, automock)]
pub trait ListPortsFeature {
    fn list<'a>(
        &'a self,
        port_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<PortDto>, PortError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ListPorts<R: PortRepository> {
    port_repository: Arc<R>,
}

impl<R: PortRepository> ListPorts<R> {
    pub fn new(port_repository: Arc<R>) -> Self {
        Self { port_repository }
    }
}

impl<R> ListPortsFeature for ListPorts<R>
where
    R: PortRepository + Send + Sync,
{
    fn list<'a>(
        &'a self,
        host_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<PortDto>, PortError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(|cursor| decode_cursor(&cursor)).transpose()?;
            let limit = PORTS_PAGE_SIZE_LIMIT + 1;
            let mut ports = self
                .port_repository
                .list_ports(&host_id, maybe_cursor_id, limit)
                .await?;
            if ports.len() == limit as usize {
                has_next_page = true;
                ports.pop();
            }
            let port_dtos = ports
                .iter()
                .map(|p| PortDto {
                    id: p.id,
                    port: p.port,
                    protocol: p.protocol.clone(),
                    state: p.state.clone(),
                    service: p.service.clone(),
                    product: p.product.clone(),
                    version: p.version.clone(),
                })
                .collect();
            Ok(Page {
                data: port_dtos,
                page_info: PageInfo {
                    has_next_page,
                    end_cursor: match has_next_page {
                        true => Some(encode_cursor(
                            &ports.last().ok_or(PortError::NoLastCursor)?.id,
                        )),
                        false => None,
                    },
                },
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::PORTS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::port::{model::PortEntity, repository::MockPortRepository},
        transport::pagination::PageInfo,
    };

    use super::*;

    #[tokio::test]
    async fn test_list_ports_no_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_port_id = Uuid::now_v7();
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port_entities_size = PORTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_port_entities = vec![expected_port; expected_port_entities_size];
        let expected_port_dto = PortDto {
            id: expected_port_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
        };
        let expected_ports = vec![expected_port_dto; expected_port_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
        let expected_page = Page::<PortDto> {
            data: expected_ports,
            page_info: expected_page_info,
        };

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository
            .expect_list_ports()
            .returning(move |_, _, _| {
                let port_entities = expected_port_entities.clone();
                Box::pin(async { Ok(port_entities) })
            });

        let future = ListPorts::new(Arc::new(mock_port_repository));

        // Act
        let actual_page = future
            .list(expected_host_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_ports_with_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_port_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_port_id);
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port_entities_size = PORTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_port_entities = vec![expected_port; expected_port_entities_size + 1];
        let expected_port_dto = PortDto {
            id: expected_port_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
        };
        let expected_ports = vec![expected_port_dto; expected_port_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<PortDto> {
            data: expected_ports,
            page_info: expected_page_info,
        };

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository
            .expect_list_ports()
            .returning(move |_, _, _| {
                let port_entities = expected_port_entities.clone();
                Box::pin(async { Ok(port_entities) })
            });

        let future = ListPorts::new(Arc::new(mock_port_repository));

        // Act
        let actual_page = future
            .list(expected_host_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_ports_no_cursor() {
        // Arrange
        let expected_port_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_port_id);
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port_entities_size = PORTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_port_entities = vec![expected_port; expected_port_entities_size + 1];
        let expected_port_dto = PortDto {
            id: expected_port_id,
            port: expected_port_number,
            protocol: None,
            state: None,
            service: None,
            product: None,
            version: None,
        };
        let expected_ports = vec![expected_port_dto; expected_port_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<PortDto> {
            data: expected_ports,
            page_info: expected_page_info,
        };

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository
            .expect_list_ports()
            .returning(move |_, _, _| {
                let port_entities = expected_port_entities.clone();
                Box::pin(async { Ok(port_entities) })
            });

        let future = ListPorts::new(Arc::new(mock_port_repository));

        // Act
        let actual_page = future.list(expected_host_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_ports_not_found() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_port_id = Uuid::now_v7();
        let expected_host_id = Uuid::now_v7();
        let expected_port_number = 22;
        let expected_port_entities_size = PORTS_PAGE_SIZE_LIMIT as usize;
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

        let mut mock_port_repository = MockPortRepository::new();
        mock_port_repository
            .expect_list_ports()
            .returning(|_, _, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let future = ListPorts::new(Arc::new(mock_port_repository));

        // Act
        let actual_result = future
            .list(expected_host_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(PortError::NotFound)));
    }
}
