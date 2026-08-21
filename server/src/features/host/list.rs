use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::{
    constants::HOSTS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::host::{dto::HostDto, error::HostError, repository::HostRepository},
    transport::pagination::{Page, PageInfo},
};

pub trait ListHostsFeature {
    fn list<'a>(
        &'a self,
        scan_run_id: &'a Uuid,
        cursor_id: Option<&'a str>,
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
        scan_run_id: &'a Uuid,
        cursor_id: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<HostDto>, HostError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(decode_cursor).transpose()?;
            let mayme_cursor_id_ref = maybe_cursor_id.as_ref();
            let limit = HOSTS_PAGE_SIZE_LIMIT + 1;
            let mut hosts = self
                .host_repository
                .list_hosts(scan_run_id, mayme_cursor_id_ref, limit)
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
    use std::sync::Mutex;

    use sqlx::types::ipnetwork::IpNetwork;

    use crate::{
        constants::HOSTS_PAGE_SIZE_LIMIT, domain::cursor::encode_cursor,
        features::host::model::HostEntity, transport::pagination::PageInfo,
    };

    use super::*;

    struct MockHostRepository {
        error: Mutex<Option<sqlx::Error>>,
        host_entity: HostEntity,
        size: usize,
    }

    impl HostRepository for MockHostRepository {
        async fn get_host(&self, _: &Uuid) -> Result<HostEntity, sqlx::Error> {
            todo!()
        }

        async fn list_hosts(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<HostEntity>, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(vec![self.host_entity.clone(); self.size]),
            }
        }
    }

    #[tokio::test]
    async fn test_list_hosts_no_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
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
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(None),
            host_entity: expected_host,
            size: expected_host_entities_size,
        };
        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature
            .list(&expected_scan_run_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_hosts_with_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
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
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(None),
            host_entity: expected_host,
            size: expected_host_entities_size + 1,
        };
        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature
            .list(&expected_scan_run_id, Some(expected_cursor_id))
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
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(None),
            host_entity: expected_host,
            size: expected_host_entities_size + 1,
        };
        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_page = feature.list(&expected_scan_run_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_hosts_not_found() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
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
        let mock_host_repository = MockHostRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            host_entity: expected_host,
            size: expected_host_entities_size,
        };
        let feature = ListHosts::new(Arc::new(mock_host_repository));

        // Act
        let actual_result = feature
            .list(&expected_scan_run_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(HostError::NotFound)));
    }
}
