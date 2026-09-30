use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::{
    constants::SCAN_RUNS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::scan_run::{dto::ScanRunDto, error::ScanRunError, repository::ScanRunRepository},
    transport::pagination::{Page, PageInfo},
};

#[cfg_attr(test, automock)]
pub trait ListScanRunsFeature {
    fn list<'a>(
        &'a self,
        scan_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ScanRunDto>, ScanRunError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ListScanRuns<R: ScanRunRepository> {
    scan_run_repository: Arc<R>,
}

impl<R: ScanRunRepository> ListScanRuns<R> {
    pub fn new(scan_run_repository: Arc<R>) -> Self {
        Self {
            scan_run_repository,
        }
    }
}

impl<R> ListScanRunsFeature for ListScanRuns<R>
where
    R: ScanRunRepository + Send + Sync,
{
    fn list<'a>(
        &'a self,
        scan_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ScanRunDto>, ScanRunError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(|cursor| decode_cursor(&cursor)).transpose()?;
            let limit = SCAN_RUNS_PAGE_SIZE_LIMIT + 1;
            let mut scan_runs = self
                .scan_run_repository
                .list_scan_runs(&scan_id, maybe_cursor_id, limit)
                .await?;
            if scan_runs.len() == limit as usize {
                has_next_page = true;
                scan_runs.pop();
            }
            let scan_run_dtos = scan_runs
                .iter()
                .map(|r| ScanRunDto {
                    id: r.id,
                    scan_id: r.scan_id,
                    created_at: r.created_at,
                })
                .collect();
            Ok(Page {
                data: scan_run_dtos,
                page_info: PageInfo {
                    has_next_page,
                    end_cursor: match has_next_page {
                        true => Some(encode_cursor(
                            &scan_runs.last().ok_or(ScanRunError::NoLastCursor)?.id,
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
    use time::macros::datetime;

    use crate::{
        constants::SCAN_RUNS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::scan_run::{model::ScanRunEntity, repository::MockScanRunRepository},
        transport::pagination::PageInfo,
    };

    use super::*;

    #[tokio::test]
    async fn test_list_scan_runs_no_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_scan_run_id = Uuid::now_v7();
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run_entities_size = SCAN_RUNS_PAGE_SIZE_LIMIT as usize;
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_run_entities = vec![expected_scan_run; expected_scan_run_entities_size];
        let expected_scan_run_dto = ScanRunDto {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_runs = vec![expected_scan_run_dto; expected_scan_run_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
        let expected_page = Page::<ScanRunDto> {
            data: expected_scan_runs,
            page_info: expected_page_info,
        };

        let mut mock_scan_run_repository = MockScanRunRepository::new();
        mock_scan_run_repository
            .expect_list_scan_runs()
            .returning(move |_, _, _| {
                let scan_run_entities = expected_scan_run_entities.clone();
                Box::pin(async { Ok(scan_run_entities) })
            });

        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature
            .list(expected_scan_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scan_runs_with_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_scan_run_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_scan_run_id);
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run_entities_size = SCAN_RUNS_PAGE_SIZE_LIMIT as usize;
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_run_entities =
            vec![expected_scan_run; expected_scan_run_entities_size + 1];
        let expected_scan_run_dto = ScanRunDto {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_runs = vec![expected_scan_run_dto; expected_scan_run_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ScanRunDto> {
            data: expected_scan_runs,
            page_info: expected_page_info,
        };

        let mut mock_scan_run_repository = MockScanRunRepository::new();
        mock_scan_run_repository
            .expect_list_scan_runs()
            .returning(move |_, _, _| {
                let scan_run_entities = expected_scan_run_entities.clone();
                Box::pin(async { Ok(scan_run_entities) })
            });

        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature
            .list(expected_scan_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scan_runs_no_cursor() {
        // Arrange
        let expected_scan_run_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_scan_run_id);
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run_entities_size = SCAN_RUNS_PAGE_SIZE_LIMIT as usize;
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_run_entities =
            vec![expected_scan_run; expected_scan_run_entities_size + 1];
        let expected_scan_run_dto = ScanRunDto {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let expected_scan_runs = vec![expected_scan_run_dto; expected_scan_run_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ScanRunDto> {
            data: expected_scan_runs,
            page_info: expected_page_info,
        };

        let mut mock_scan_run_repository = MockScanRunRepository::new();
        mock_scan_run_repository
            .expect_list_scan_runs()
            .returning(move |_, _, _| {
                let scan_run_entities = expected_scan_run_entities.clone();
                Box::pin(async { Ok(scan_run_entities) })
            });

        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature.list(expected_scan_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scan_runs_not_found() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_scan_id = Uuid::now_v7();

        let mut mock_scan_run_repository = MockScanRunRepository::new();
        mock_scan_run_repository
            .expect_list_scan_runs()
            .returning(|_, _, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_result = feature
            .list(expected_scan_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(ScanRunError::NotFound)));
    }
}
