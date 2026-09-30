use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::{
    constants::SCANS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::scan::{dto::ScanDto, error::ScanError, repository::ScanRepository},
    transport::pagination::{Page, PageInfo},
};

#[cfg_attr(test, automock)]
pub trait ListScansFeature {
    fn list<'a>(
        &'a self,
        project_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ScanDto>, ScanError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ListScans<S: ScanRepository> {
    scan_repository: Arc<S>,
}

impl<S: ScanRepository> ListScans<S> {
    pub fn new(scan_repository: Arc<S>) -> Self {
        Self { scan_repository }
    }
}

impl<S> ListScansFeature for ListScans<S>
where
    S: ScanRepository + Send + Sync,
{
    fn list<'a>(
        &'a self,
        project_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ScanDto>, ScanError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(|cursor| decode_cursor(&cursor)).transpose()?;
            let limit = SCANS_PAGE_SIZE_LIMIT + 1;
            let mut scans = self
                .scan_repository
                .list_scans(&project_id, maybe_cursor_id, limit)
                .await?;
            if scans.len() == limit as usize {
                has_next_page = true;
                scans.pop();
            }
            let scan_dtos = scans
                .iter()
                .map(|s| ScanDto {
                    id: s.id,
                    target: s.target,
                    schedule: s.schedule.clone(),
                    created_at: s.created_at,
                })
                .collect();
            Ok(Page {
                data: scan_dtos,
                page_info: PageInfo {
                    has_next_page,
                    end_cursor: match has_next_page {
                        true => Some(encode_cursor(
                            &scans.last().ok_or(ScanError::NoLastCursor)?.id,
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
    use time::macros::datetime;

    use crate::{
        constants::SCANS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::scan::{model::ScanEntity, repository::MockScanRepository},
        transport::pagination::PageInfo,
    };

    use super::*;

    #[tokio::test]
    async fn test_list_scans_no_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_scan_id = Uuid::now_v7();
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_schedule = Some(String::from("0 * * * * *"));
        let expected_status = crate::features::scan::model::ScanStatus::Scheduled;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = Some(datetime!(2019-02-01 0:00 UTC));
        let expected_scan_entities_size = SCANS_PAGE_SIZE_LIMIT as usize;
        let expected_scan = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_status,
            schedule: expected_schedule.clone(),
            created_at: expected_created_at,
            next_run_at: expected_next_run_at,
        };
        let expected_scan_entities = vec![expected_scan; expected_scan_entities_size];
        let expected_scan_dto = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: expected_schedule,
            created_at: expected_created_at,
        };
        let expected_scans = vec![expected_scan_dto; expected_scan_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
        let expected_page = Page::<ScanDto> {
            data: expected_scans,
            page_info: expected_page_info,
        };

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_list_scans()
            .returning(move |_, _, _| {
                let scan_entites = expected_scan_entities.clone();
                Box::pin(async { Ok(scan_entites) })
            });

        let feature = ListScans::new(Arc::new(mock_scan_repository));

        // Act
        let actual_page = feature
            .list(expected_project_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scans_with_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_scan_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_scan_id);
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_schedule = Some(String::from("0 * * * * *"));
        let expected_status = crate::features::scan::model::ScanStatus::Scheduled;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = Some(datetime!(2019-02-01 0:00 UTC));
        let expected_scan_entities_size = SCANS_PAGE_SIZE_LIMIT as usize;
        let expected_scan = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_status,
            schedule: expected_schedule.clone(),
            created_at: expected_created_at,
            next_run_at: expected_next_run_at,
        };
        let expected_scan_entities = vec![expected_scan; expected_scan_entities_size + 1];
        let expected_scan_dto = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: expected_schedule,
            created_at: expected_created_at,
        };
        let expected_scans = vec![expected_scan_dto; expected_scan_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ScanDto> {
            data: expected_scans,
            page_info: expected_page_info,
        };

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_list_scans()
            .returning(move |_, _, _| {
                let scan_entites = expected_scan_entities.clone();
                Box::pin(async { Ok(scan_entites) })
            });

        let feature = ListScans::new(Arc::new(mock_scan_repository));

        // Act
        let actual_page = feature
            .list(expected_project_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scans_no_cursor() {
        // Arrange
        let expected_scan_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_scan_id);
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_schedule = Some(String::from("0 * * * * *"));
        let expected_status = crate::features::scan::model::ScanStatus::Scheduled;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = Some(datetime!(2019-02-01 0:00 UTC));
        let expected_scan_entities_size = SCANS_PAGE_SIZE_LIMIT as usize;
        let expected_scan = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_status,
            schedule: expected_schedule.clone(),
            created_at: expected_created_at,
            next_run_at: expected_next_run_at,
        };
        let expected_scan_entities = vec![expected_scan; expected_scan_entities_size + 1];
        let expected_scan_dto = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: expected_schedule,
            created_at: expected_created_at,
        };
        let expected_scans = vec![expected_scan_dto; expected_scan_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ScanDto> {
            data: expected_scans,
            page_info: expected_page_info,
        };

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_list_scans()
            .returning(move |_, _, _| {
                let scan_entites = expected_scan_entities.clone();
                Box::pin(async { Ok(scan_entites) })
            });

        let feature = ListScans::new(Arc::new(mock_scan_repository));

        // Act
        let actual_page = feature.list(expected_project_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scans_not_found() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_project_id = Uuid::now_v7();

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_list_scans()
            .returning(|_, _, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = ListScans::new(Arc::new(mock_scan_repository));

        // Act
        let actual_result = feature
            .list(expected_project_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(ScanError::NotFound)));
    }
}
