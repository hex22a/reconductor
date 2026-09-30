use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use cron::Schedule;
use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    features::scan::{
        dto::ScanDto,
        error::ScanError,
        model::{ScanEntity, ScanInsert},
        repository::ScanRepository,
    },
    infra::{message_queue::publisher::Publisher, scheduler::SchedulerService},
};

#[cfg_attr(test, automock)]
pub trait CreateScanFeature {
    fn create<'a>(
        &'a self,
        project_id: Uuid,
        target: IpNetwork,
        schedule: Option<Schedule>,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + 'a>>;
}

pub struct CreateScan<R: ScanRepository, P: Publisher, S: SchedulerService> {
    scan_repository: Arc<R>,
    publisher: Arc<P>,
    scheduler: Arc<S>,
}

impl<R: ScanRepository, P: Publisher, S: SchedulerService> CreateScan<R, P, S> {
    pub fn new(scan_repository: Arc<R>, publisher: Arc<P>, scheduler: Arc<S>) -> Self {
        Self {
            scan_repository,
            publisher,
            scheduler,
        }
    }
}

impl<R, P, S> CreateScanFeature for CreateScan<R, P, S>
where
    R: ScanRepository + Send + Sync,
    P: Publisher + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    fn create(
        &self,
        project_id: Uuid,
        target: IpNetwork,
        schedule: Option<Schedule>,
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
                schedule: schedule.map(|s| s.to_string()),
                next_run_at,
            };
            let ScanEntity {
                id,
                target,
                schedule,
                created_at,
                ..
            } = self.scan_repository.create_scan(scan_insert).await?;
            self.publisher.publish_scan(&id, &target).await?;
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
    use std::str::FromStr;

    use mockall::predicate::eq;
    use time::macros::datetime;

    use crate::{
        features::scan::{
            model::{ScanEntity, ScanStatus},
            repository::MockScanRepository,
        },
        infra::{message_queue::publisher::MockPublisher, scheduler::MockSchedulerService},
    };

    use super::*;

    #[tokio::test]
    async fn test_create_scan() {
        // Arrange
        let expected_scan_id = Uuid::now_v7();
        let expected_project_id = Uuid::now_v7();
        let expected_target = "192.168.0.1".parse().unwrap();
        let expected_scan_status = ScanStatus::Scheduled;
        let expected_schedule = Schedule::from_str("* * * * * *").unwrap();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_next_run_at = datetime!(2019-01-02 0:00 UTC);
        let expected_scan_entity = ScanEntity {
            id: expected_scan_id,
            project_id: expected_project_id,
            target: expected_target,
            status: expected_scan_status,
            schedule: Some(expected_schedule.to_string()),
            created_at: expected_created_at,
            next_run_at: Some(expected_next_run_at),
        };
        let expected_scan = ScanDto {
            id: expected_scan_id,
            target: expected_target,
            schedule: Some(expected_schedule.to_string()),
            created_at: expected_created_at,
        };

        let mut mock_mq_publisher = MockPublisher::new();
        mock_mq_publisher
            .expect_publish_scan()
            .with(eq(expected_scan_id), eq(expected_target))
            .returning(|_, _| Box::pin(async { Ok(()) }));
        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_create_scan()
            .returning(move |_| {
                let scan_entity = expected_scan_entity.clone();
                Box::pin(async { Ok(scan_entity) })
            });
        let mut mock_scheduler_service = MockSchedulerService::new();
        mock_scheduler_service
            .expect_calculate_next_run()
            .return_const(Ok(expected_next_run_at));

        let feature = CreateScan::new(
            Arc::new(mock_scan_repository),
            Arc::new(mock_mq_publisher),
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
