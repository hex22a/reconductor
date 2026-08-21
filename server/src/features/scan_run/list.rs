use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::{
    constants::SCAN_RUNS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::scan_run::{dto::ScanRunDto, error::ScanRunError, repository::ScanRunRepository},
    transport::pagination::{Page, PageInfo},
};

pub trait ListScanRunsFeature {
    fn list<'a>(
        &'a self,
        scan_id: &'a Uuid,
        cursor_id: Option<&'a str>,
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
        scan_id: &'a Uuid,
        cursor_id: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ScanRunDto>, ScanRunError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(decode_cursor).transpose()?;
            let maybe_cursor_id_ref = maybe_cursor_id.as_ref();
            let limit = SCAN_RUNS_PAGE_SIZE_LIMIT + 1;
            let mut scan_runs = self
                .scan_run_repository
                .list_scan_runs(scan_id, maybe_cursor_id_ref, limit)
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
    use std::sync::Mutex;

    use time::macros::datetime;

    use crate::{
        constants::SCAN_RUNS_PAGE_SIZE_LIMIT, domain::cursor::encode_cursor,
        features::scan_run::model::ScanRunEntity, transport::pagination::PageInfo,
    };

    use super::*;

    struct MockScanRunRepository {
        error: Mutex<Option<sqlx::Error>>,
        scan_run_entity: ScanRunEntity,
        size: usize,
    }

    impl ScanRunRepository for MockScanRunRepository {
        async fn get_scan_run(&self, _: &Uuid) -> Result<ScanRunEntity, sqlx::Error> {
            todo!()
        }

        async fn list_scan_runs(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<ScanRunEntity>, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(vec![self.scan_run_entity.clone(); self.size]),
            }
        }
    }

    #[tokio::test]
    async fn test_list_scan_runs_no_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
        let expected_scan_run_id = Uuid::now_v7();
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run_entities_size = SCAN_RUNS_PAGE_SIZE_LIMIT as usize;
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
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
        let mock_scan_run_repository = MockScanRunRepository {
            error: Mutex::new(None),
            scan_run_entity: expected_scan_run,
            size: expected_scan_run_entities_size,
        };
        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature
            .list(&expected_scan_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_scan_runs_with_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
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
        let mock_scan_run_repository = MockScanRunRepository {
            error: Mutex::new(None),
            scan_run_entity: expected_scan_run,
            size: expected_scan_run_entities_size + 1,
        };
        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature
            .list(&expected_scan_id, Some(expected_cursor_id))
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
        let mock_scan_run_repository = MockScanRunRepository {
            error: Mutex::new(None),
            scan_run_entity: expected_scan_run,
            size: expected_scan_run_entities_size + 1,
        };
        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_page = feature.list(&expected_scan_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }
    #[tokio::test]
    async fn test_list_scan_runs_not_found() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
        let expected_scan_run_id = Uuid::now_v7();
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run_entities_size = SCAN_RUNS_PAGE_SIZE_LIMIT as usize;
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let mock_scan_run_repository = MockScanRunRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            scan_run_entity: expected_scan_run,
            size: expected_scan_run_entities_size,
        };
        let feature = ListScanRuns::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_result = feature
            .list(&expected_scan_id, Some(expected_cursor_id))
            .await;

        // Assert
        assert!(matches!(actual_result, Err(ScanRunError::NotFound)));
    }
}
