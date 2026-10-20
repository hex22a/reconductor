use std::str::FromStr;

use cron::Schedule;
use reconductor_messaging::publisher::Publisher;
use reconductor_schedule::SchedulerService;
use tokio::time::{Duration, interval};
use tracing::{error, info};

use crate::features::scan::{error::ScanError, repository::ScanRepository};

pub trait PollerFeature {
    fn run(&self) -> impl Future<Output = Result<(), ScanError>>;
    fn poll(&self) -> impl Future<Output = Result<(), ScanError>>;
}

pub struct ScanPoller<R: ScanRepository, P: Publisher, S: SchedulerService> {
    repository: R,
    publisher: P,
    scheduler: S,
    poll_interval: Duration,
}

impl<R: ScanRepository, P: Publisher, S: SchedulerService> ScanPoller<R, P, S> {
    pub fn new(repository: R, publisher: P, scheduler: S, poll_interval_secs: u64) -> Self {
        Self {
            repository,
            publisher,
            scheduler,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }
}

impl<R: ScanRepository, P: Publisher, S: SchedulerService> PollerFeature for ScanPoller<R, P, S> {
    async fn run(&self) -> Result<(), ScanError> {
        info!(
            "Scheduler started, polling every {}s",
            self.poll_interval.as_secs()
        );
        let mut ticker = interval(self.poll_interval);
        loop {
            ticker.tick().await;
            if let Err(e) = self.poll().await {
                error!("Poller error: {}", e);
            }
        }
    }

    async fn poll(&self) -> Result<(), ScanError> {
        let due_scans = self.repository.fetch_due_scans().await?;

        if due_scans.is_empty() {
            info!("No due scans");
            return Ok(());
        }

        info!("Found {} due scan(s)", due_scans.len());

        for scan in due_scans {
            let Some(schedule) = &scan.schedule else {
                continue;
            };

            match self.publisher.publish_scan(&scan.id, &scan.target).await {
                Ok(_) => {
                    info!("Published scan {:?} for target {:?}", scan.id, scan.target);
                    let schedule = Schedule::from_str(schedule)?;
                    match self.scheduler.calculate_next_run(&schedule) {
                        Ok(next_run) => {
                            if let Err(e) =
                                self.repository.update_next_run(&scan.id, next_run).await
                            {
                                error!("Failed to update next_run for scan {:?}: {}", scan.id, e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to calculate next run for scan {:?}: {}", scan.id, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to publish scan {:?}: {}", scan.id, e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cron::Schedule;
    use mockall::{mock, predicate::eq};
    use reconductor_core::{ScanId, ScanTarget};
    use reconductor_messaging::error::MqError;
    use reconductor_schedule::ScheduleError;
    use sqlx::types::{
        ipnetwork::{IpNetwork, Ipv4Network},
        time::OffsetDateTime,
    };
    use uuid::Uuid;

    use crate::features::scan::{model::DueScan, repository::MockScanRepository};

    use super::*;

    mock! {
        ScanPublisher {}

        impl Publisher for ScanPublisher {
            fn publish_scan(
                &self,
                scan_id: &ScanId,
                target: &ScanTarget,
            ) -> impl Future<Output = Result<(), MqError>> + Send;
        }
    }

    mock! {
        Scheduler {}

        impl SchedulerService for Scheduler {
            fn calculate_next_run(&self, schedule: &Schedule) -> Result<OffsetDateTime, ScheduleError>;
        }
    }

    #[tokio::test]
    async fn test_poll_no_due_scans() {
        // Arrange
        let expected_poll_interval_secs: u64 = 30;
        let expected_due_scans: Vec<DueScan> = vec![];

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_fetch_due_scans()
            .returning(move || {
                let due_scans = expected_due_scans.clone();
                Box::pin(async { Ok(due_scans) })
            });
        mock_scan_repository.expect_update_next_run().never();

        let mock_scan_publisher = MockScanPublisher::new();
        let mock_scheduler_service = MockScheduler::new();

        let scheduler = ScanPoller::new(
            mock_scan_repository,
            mock_scan_publisher,
            mock_scheduler_service,
            expected_poll_interval_secs,
        );

        // Act
        scheduler.poll().await.unwrap();

        // Assert
    }

    #[tokio::test]
    async fn test_poll_happy_path() {
        // Arrange
        let expected_scan_id = ScanId::new(Uuid::now_v7());
        let expected_target = ScanTarget::new(IpNetwork::V4(
            Ipv4Network::from_str("192.168.0.0/16").unwrap(),
        ));
        let expected_schedule_string: String = String::from_str("0 5 * * * *").unwrap();
        let expected_schedule = Schedule::from_str(&expected_schedule_string).unwrap();
        let expected_poll_interval_secs: u64 = 30;
        let expected_next_run: OffsetDateTime = OffsetDateTime::now_utc();
        let expected_due_scans: Vec<DueScan> = vec![DueScan {
            id: expected_scan_id.clone(),
            target: expected_target.clone(),
            schedule: Some(expected_schedule_string),
        }];

        let mut mock_scan_repository = MockScanRepository::new();
        mock_scan_repository
            .expect_fetch_due_scans()
            .returning(move || {
                let due_scans = expected_due_scans.clone();
                Box::pin(async { Ok(due_scans) })
            });
        mock_scan_repository
            .expect_update_next_run()
            .with(eq(expected_scan_id.clone()), eq(expected_next_run))
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let mut mock_scan_publisher = MockScanPublisher::new();
        mock_scan_publisher
            .expect_publish_scan()
            .with(eq(expected_scan_id), eq(expected_target))
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let mut mock_scheduler_service = MockScheduler::new();
        mock_scheduler_service
            .expect_calculate_next_run()
            .with(eq(expected_schedule))
            .return_const(Ok(expected_next_run));

        let scheduler = ScanPoller::new(
            mock_scan_repository,
            mock_scan_publisher,
            mock_scheduler_service,
            expected_poll_interval_secs,
        );
        // Act
        scheduler.poll().await.unwrap();

        // Assert
    }
}
