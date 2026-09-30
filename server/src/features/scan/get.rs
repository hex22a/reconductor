use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::features::scan::{
    dto::ScanDto, error::ScanError, model::ScanEntity, repository::ScanRepository,
};

#[cfg_attr(test, automock)]
pub trait GetScanFeature {
    fn get<'a>(
        &'a self,
        scan_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + 'a>>;
}

pub struct GetScan<R: ScanRepository> {
    scan_repository: Arc<R>,
}

impl<R: ScanRepository> GetScan<R> {
    pub fn new(scan_repository: Arc<R>) -> Self {
        Self { scan_repository }
    }
}

impl<R> GetScanFeature for GetScan<R>
where
    R: ScanRepository + Send + Sync,
{
    fn get<'a>(
        &'a self,
        scan_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + 'a>> {
        Box::pin(async move {
            let ScanEntity {
                id,
                target,
                schedule,
                created_at,
                ..
            } = self.scan_repository.get_scan(&scan_id).await?;
            Ok(ScanDto {
                id,
                target,
                schedule,
                created_at,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use sqlx::types::ipnetwork::IpNetwork;
    use time::macros::datetime;

    use crate::features::scan::{model::ScanEntity, repository::MockScanRepository};

    use super::*;

    #[tokio::test]
    async fn test_get_scan() {
        // Arrange
        let expected_scan_id = Uuid::now_v7();
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_schedule = Some(String::from("0 * * * * *"));
        let expected_status = crate::features::scan::model::ScanStatus::Scheduled;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = Some(datetime!(2019-02-01 0:00 UTC));
        let expected_scan = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_status,
            schedule: expected_schedule.clone(),
            created_at: expected_created_at,
            next_run_at: expected_next_run_at,
        };
        let expected_scan_dto = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: expected_schedule,
            created_at: expected_created_at,
        };

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository.expect_get_scan().returning(move |_| {
            let scan = expected_scan.clone();
            Box::pin(async { Ok(scan) })
        });

        let feature = GetScan::new(Arc::new(mock_scan_repository));

        // Act
        let actual_scan_dto = feature.get(expected_scan_id).await.unwrap();

        // Assert
        assert_eq!(actual_scan_dto, expected_scan_dto);
    }

    #[tokio::test]
    async fn test_get_scan_not_found() {
        // Arrange
        let expected_scan_id = Uuid::now_v7();

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_get_scan()
            .returning(|_| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = GetScan::new(Arc::new(mock_scan_repository));

        // Act
        let actual_result = feature.get(expected_scan_id).await;

        // Assert
        assert!(matches!(actual_result, Err(ScanError::NotFound)));
    }
}
