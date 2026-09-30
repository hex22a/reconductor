use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::{
    constants::HOSTS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::host::{dto::HostDto, error::HostError, repository::HostRepository},
    transport::pagination::{Page, PageInfo},
};

#[cfg_attr(test, automock)]
pub trait ListHostsFeature {
    fn list<'a>(
        &'a self,
        scan_run_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<HostDto>, HostError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ListHosts<R: HostRepository> {
    host_repository: Arc<R>,
}

impl<R: HostRepository> ListHosts<R> {
    pub fn new(host_repository: Arc<R>) -> Self {
        Self { host_repository }
    }
}

impl<R> ListHostsFeature for ListHosts<R>
where
    R: HostRepository + Send + Sync,
{
    fn list<'a>(
        &'a self,
        scan_run_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<HostDto>, HostError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(|cursor| decode_cursor(&cursor)).transpose()?;
            let limit = HOSTS_PAGE_SIZE_LIMIT + 1;
            let mut hosts = self
                .host_repository
                .list_hosts(&scan_run_id, maybe_cursor_id, limit)
                .await?;
            if hosts.len() == limit as usize {
                has_next_page = true;
                hosts.pop();
            }
            let host_dtos = hosts
                .iter()
                .map(|h| HostDto {
                    id: h.id,
                    ip: h.ip.ip(),
                    mac: h.mac,
                    vendor: h.vendor.clone(),
                    hostname: h.hostname.clone(),
                    os_match: h.os_match.clone(),
                    os_accuracy: h.os_accuracy.map(|n| format!("{}", n)),
                })
                .collect();
            Ok(Page {
                data: host_dtos,
                page_info: PageInfo {
                    has_next_page,
                    end_cursor: match has_next_page {
                        true => Some(encode_cursor(
                            &hosts.last().ok_or(HostError::NoLastCursor)?.id,
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
    use sqlx::types::ipnetwork::IpNetwork;

    use crate::{
        constants::HOSTS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::host::{model::HostEntity, repository::MockHostRepository},
        transport::pagination::PageInfo,
    };

    use super::*;

    #[tokio::test]
    async fn test_list_hosts_no_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_host_id = Uuid::now_v7();
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host_entities_size = HOSTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_host_entities = vec![expected_host; expected_host_entities_size];
        let expected_host_dto = HostDto {
            id: expected_host_id,
            ip: expected_ip.ip(),
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let expected_hosts = vec![expected_host_dto; expected_host_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
        let expected_page = Page::<HostDto> {
            data: expected_hosts,
            page_info: expected_page_info,
        };

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository
            .expect_list_hosts()
            .returning(move |_, _, _| {
                let host_enitities = expected_host_entities.clone();
                Box::pin(async { Ok(host_enitities) })
            });

        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature
            .list(expected_scan_run_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_hosts_with_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_host_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_host_id);
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host_entities_size = HOSTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_host_entities = vec![expected_host; expected_host_entities_size + 1];
        let expected_host_dto = HostDto {
            id: expected_host_id,
            ip: expected_ip.ip(),
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let expected_hosts = vec![expected_host_dto; expected_host_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<HostDto> {
            data: expected_hosts,
            page_info: expected_page_info,
        };

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository
            .expect_list_hosts()
            .returning(move |_, _, _| {
                let host_enitities = expected_host_entities.clone();
                Box::pin(async { Ok(host_enitities) })
            });

        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature
            .list(expected_scan_run_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_hosts_no_cursor() {
        // Arrange
        let expected_host_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_host_id);
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host_entities_size = HOSTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_host_entities = vec![expected_host; expected_host_entities_size + 1];
        let expected_host_dto = HostDto {
            id: expected_host_id,
            ip: expected_ip.ip(),
            mac: None,
            vendor: None,
            hostname: None,
            os_match: None,
            os_accuracy: None,
        };
        let expected_hosts = vec![expected_host_dto; expected_host_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<HostDto> {
            data: expected_hosts,
            page_info: expected_page_info,
        };

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository
            .expect_list_hosts()
            .returning(move |_, _, _| {
                let host_enitities = expected_host_entities.clone();
                Box::pin(async { Ok(host_enitities) })
            });

        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature.list(expected_scan_run_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_hosts_not_found() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_host_id = Uuid::now_v7();
        let expected_scan_run_id = Uuid::now_v7();
        let expected_ip: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_host_entities_size = HOSTS_PAGE_SIZE_LIMIT as usize;
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

        let mut mock_host_repository = MockHostRepository::new();
        mock_host_repository
            .expect_list_hosts()
            .returning(|_, _, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_result = feature
            .list(expected_scan_run_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(HostError::NotFound)));
    }
}
