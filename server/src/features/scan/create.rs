use std::{pin::Pin, sync::Arc};

use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    features::scan::{
        dto::ScanDto, error::ScanError, model::ScanInsert, repository::ScanRepository,
    },
    infra::scheduler::SchedulerService,
};

pub(crate) trait CreateScanFeature {
    fn create(
        &self,
        project_id: Uuid,
        target: IpNetwork,
        schedule: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + '_>>;
}

pub(crate) struct CreateScan<R: ScanRepository, S: SchedulerService> {
    scan_repository: Arc<R>,
    scheduler: Arc<S>,
}

impl<R: ScanRepository, S: SchedulerService> CreateScan<R, S> {
    pub(crate) fn new(scan_repository: Arc<R>, scheduler: Arc<S>) -> Self {
        Self {
            scan_repository,
            scheduler,
        }
    }
}

impl<R, S> CreateScanFeature for CreateScan<R, S>
where
    R: ScanRepository + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    fn create(
        &self,
        project_id: Uuid,
        target: IpNetwork,
        schedule: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + '_>> {
        Box::pin(async move {
            let next_run_at: Option<OffsetDateTime> = if let Some(s) = &schedule {
                Some(self.scheduler.calculate_next_run(s)?)
            } else {
                None
            };
            let scan_insert = ScanInsert {
                project_id,
                target,
                schedule,
                next_run_at,
            };
            let scan = self.scan_repository.create_scan(scan_insert).await?;
            Ok(ScanDto {
                id: scan.id,
                target,
                schedule: scan.schedule,
                created_at: scan.created_at,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use time::{OffsetDateTime, macros::datetime};

    use crate::{
        features::scan::model::{ScanEntity, ScanInsert, ScanStatus},
        infra::scheduler::ScheduleError,
    };

    use super::*;

    struct MockScanReposotry {
        error: Mutex<Option<sqlx::Error>>,
        return_value: ScanEntity,
    }

    struct MockScheduler {
        error: Mutex<Option<ScheduleError>>,
        return_value: OffsetDateTime,
    }

    impl ScanRepository for MockScanReposotry {
        async fn create_scan(&self, _: ScanInsert) -> Result<ScanEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        async fn get_scan(&self, _: &Uuid) -> Result<ScanEntity, sqlx::Error> {
            todo!()
        }

        async fn list_scans(
            &self,
            _: &Uuid,
            _: Option<&Uuid>,
            _: i64,
        ) -> Result<Vec<ScanEntity>, sqlx::Error> {
            todo!()
        }
    }

    impl SchedulerService for MockScheduler {
        fn calculate_next_run(&self, _: &str) -> Result<OffsetDateTime, ScheduleError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value),
            }
        }
    }

    #[tokio::test]
    async fn test_create_scan() {
        // Arrange
        let expected_scan_id = Uuid::now_v7();
        let expected_project_id = Uuid::now_v7();
        let expected_target = "192.168.0.1".parse().unwrap();
        let expected_scan_status = ScanStatus::Scheduled;
        let expected_schedule = "* * * * *".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = datetime!(2019-01-02 0:00 UTC);
        let expected_scan_entity = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_scan_status,
            schedule: Some(expected_schedule.clone()),
            created_at: expected_created_at,
            next_run_at: Some(expected_next_run_at),
        };
        let expected_scan = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: Some(expected_schedule.clone()),
            created_at: expected_created_at,
        };
        let mock_scan_repository = MockScanReposotry {
            error: Mutex::new(None),
            return_value: expected_scan_entity,
        };
        let mock_scheduler_service = MockScheduler {
            error: Mutex::new(None),
            return_value: expected_created_at,
        };
        let feature = CreateScan::new(
            Arc::new(mock_scan_repository),
            Arc::new(mock_scheduler_service),
        );
        // Act
        let actual_scan = feature
            .create(
                expected_project_id,
                expected_target,
                Some(expected_schedule),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_scan, expected_scan);
    }
}
