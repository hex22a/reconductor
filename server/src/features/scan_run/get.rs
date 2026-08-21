use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::features::scan_run::{
    dto::ScanRunDto, error::ScanRunError, model::ScanRunEntity, repository::ScanRunRepository,
};

pub trait GetScanRunFeature {
    fn get(
        &self,
        scan_run_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ScanRunDto, ScanRunError>> + Send + '_>>;
}

pub struct GetScanRun<R: ScanRunRepository> {
    scan_run_repository: Arc<R>,
}

impl<R: ScanRunRepository> GetScanRun<R> {
    pub fn new(scan_run_repository: Arc<R>) -> Self {
        Self {
            scan_run_repository,
        }
    }
}

impl<R> GetScanRunFeature for GetScanRun<R>
where
    R: ScanRunRepository + Send + Sync,
{
    fn get(
        &self,
        scan_run_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ScanRunDto, ScanRunError>> + Send + '_>> {
        Box::pin(async move {
            let ScanRunEntity {
                id,
                scan_id,
                created_at,
            } = self.scan_run_repository.get_scan_run(&scan_run_id).await?;
            Ok(ScanRunDto {
                id,
                scan_id,
                created_at,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use time::macros::datetime;

    use crate::features::scan_run::model::ScanRunEntity;

    use super::*;

    struct MockScanRepository {
        error: Mutex<Option<sqlx::Error>>,
        scan_run_entity: ScanRunEntity,
    }

    impl ScanRunRepository for MockScanRepository {
        async fn get_scan_run(&self, _: &Uuid) -> Result<ScanRunEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.scan_run_entity.clone()),
            }
        }

        async fn list_scan_runs(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<ScanRunEntity>, sqlx::Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_get_scan_run() {
        // Arrange
        let expected_scan_run_id = Uuid::now_v7();
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
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
        let mock_scan_run_repository = MockScanRepository {
            error: Mutex::new(None),
            scan_run_entity: expected_scan_run,
        };
        let feature = GetScanRun::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_scan_dto = feature.get(expected_scan_run_id).await.unwrap();

        // Assert
        assert_eq!(actual_scan_dto, expected_scan_run_dto);
    }

    #[tokio::test]
    async fn test_get_scan_run_not_found() {
        // Arrange
        let expected_scan_run_id = Uuid::now_v7();
        let expected_scan_id = Uuid::now_v7();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_scan_run = ScanRunEntity {
            id: expected_scan_run_id,
            scan_id: expected_scan_id,
            created_at: expected_created_at,
        };
        let mock_scan_run_repository = MockScanRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            scan_run_entity: expected_scan_run,
        };
        let feature = GetScanRun::new(Arc::new(mock_scan_run_repository));

        // Act
        let actual_result = feature.get(expected_scan_run_id).await;

        // Assert
        assert!(matches!(actual_result, Err(ScanRunError::NotFound)));
    }
}
