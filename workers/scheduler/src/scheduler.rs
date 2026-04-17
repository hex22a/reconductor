pub mod utils;

use crate::db::scan::ScanRepository;
use crate::queue::publisher::Publisher;
use crate::scheduler::utils::Utils;
use tokio::time::{Duration, interval};
use tracing::{error, info};

pub struct Scheduler<R: ScanRepository, P: Publisher, U: Utils> {
    repository: R,
    publisher: P,
    utils: U,
    poll_interval: Duration,
}

impl<R: ScanRepository, P: Publisher, U: Utils> Scheduler<R, P, U> {
    pub fn new(repository: R, publisher: P, utils: U, poll_interval_secs: u64) -> Self {
        Self {
            repository,
            publisher,
            utils,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
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

    pub async fn poll(&self) -> anyhow::Result<()> {
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

            match self.publisher.publish_scan(scan.id, &scan.target).await {
                Ok(_) => {
                    info!("Published scan {} for target {}", scan.id, scan.target);
                    match self.utils.calculate_next_run(schedule) {
                        Ok(next_run) => {
                            if let Err(e) = self.repository.update_next_run(scan.id, next_run).await
                            {
                                error!("Failed to update next_run for scan {}: {}", scan.id, e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to calculate next run for scan {}: {}", scan.id, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to publish scan {}: {}", scan.id, e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };

    use crate::db::scan::{DueScan, ScanRepository};
    use anyhow::Ok;
    use sqlx::types::{
        ipnetwork::{IpNetwork, Ipv4Network},
        time::OffsetDateTime,
    };
    use uuid::Uuid;

    use crate::{queue::publisher::Publisher, scheduler::utils::Utils};

    use super::Scheduler;

    struct MockScanRepository {
        due_scans: Vec<DueScan>,
        fetch_due_scans_calls: Arc<Mutex<Vec<()>>>,
        update_next_run_calls: Arc<Mutex<Vec<(Uuid, OffsetDateTime)>>>,
    }

    struct MockScanPublisher {
        publish_calls: Arc<Mutex<Vec<(Uuid, IpNetwork)>>>,
    }

    struct MockUtils {
        return_value: OffsetDateTime,
        calculate_next_run_calls: Arc<Mutex<Vec<String>>>,
    }

    impl ScanRepository for MockScanRepository {
        async fn fetch_due_scans(&self) -> anyhow::Result<Vec<DueScan>> {
            self.fetch_due_scans_calls.lock().unwrap().push(());
            Ok(self.due_scans.clone())
        }
        async fn update_next_run(
            &self,
            scan_id: Uuid,
            next_run_at: sqlx::types::time::OffsetDateTime,
        ) -> anyhow::Result<()> {
            self.update_next_run_calls
                .lock()
                .unwrap()
                .push((scan_id, next_run_at));
            Ok(())
        }
    }

    impl Publisher for MockScanPublisher {
        async fn publish_scan(&self, scan_id: Uuid, target: &IpNetwork) -> anyhow::Result<()> {
            self.publish_calls
                .lock()
                .unwrap()
                .push((scan_id, target.clone()));
            Ok(())
        }
    }

    impl Utils for MockUtils {
        fn calculate_next_run(&self, schedule: &str) -> anyhow::Result<OffsetDateTime> {
            self.calculate_next_run_calls
                .lock()
                .unwrap()
                .push(schedule.into());
            Ok(self.return_value)
        }
    }

    #[tokio::test]
    async fn test_poll_no_due_scans() {
        // Arrange
        let expected_poll_interval_secs: u64 = 30;
        let expected_next_run: OffsetDateTime = OffsetDateTime::now_utc();
        let expected_due_scans: Vec<DueScan> = vec![];
        let fetch_due_scans_calls: Arc<Mutex<Vec<()>>> = Arc::new(Mutex::new(vec![]));
        let update_next_run_calls: Arc<Mutex<Vec<(Uuid, OffsetDateTime)>>> =
            Arc::new(Mutex::new(vec![]));
        let publish_calls: Arc<Mutex<Vec<(Uuid, IpNetwork)>>> = Arc::new(Mutex::new(vec![]));
        let calculate_next_run_calls: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let mock_scan_repository = MockScanRepository {
            due_scans: expected_due_scans,
            fetch_due_scans_calls: fetch_due_scans_calls.clone(),
            update_next_run_calls: update_next_run_calls.clone(),
        };
        let mock_scan_publisher = MockScanPublisher {
            publish_calls: publish_calls.clone(),
        };
        let mock_utils = MockUtils {
            return_value: expected_next_run,
            calculate_next_run_calls: calculate_next_run_calls.clone(),
        };
        let scheduler = Scheduler::new(
            mock_scan_repository,
            mock_scan_publisher,
            mock_utils,
            expected_poll_interval_secs,
        );
        // Act
        let actual_result = scheduler.poll().await.unwrap();

        // Assert
        assert_eq!(actual_result, ());
        assert_eq!(fetch_due_scans_calls.lock().unwrap().len(), 1);
        assert_eq!(update_next_run_calls.lock().unwrap().len(), 0);
        assert_eq!(calculate_next_run_calls.lock().unwrap().len(), 0);
        assert_eq!(publish_calls.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_poll_happy_path() {
        // Arrange
        let expected_scan_id: Uuid = Uuid::now_v7();
        let expected_target: IpNetwork =
            IpNetwork::V4(Ipv4Network::from_str("192.168.0.0/16").unwrap());
        let expected_schedule: String = String::from_str("5 * * * *").unwrap();
        let expected_poll_interval_secs: u64 = 30;
        let expected_next_run: OffsetDateTime = OffsetDateTime::now_utc();
        let expected_due_scans: Vec<DueScan> = vec![DueScan {
            id: expected_scan_id,
            target: expected_target,
            schedule: Some(expected_schedule),
        }];
        let fetch_due_scans_calls: Arc<Mutex<Vec<()>>> = Arc::new(Mutex::new(vec![]));
        let update_next_run_calls: Arc<Mutex<Vec<(Uuid, OffsetDateTime)>>> =
            Arc::new(Mutex::new(vec![]));
        let publish_calls: Arc<Mutex<Vec<(Uuid, IpNetwork)>>> = Arc::new(Mutex::new(vec![]));
        let calculate_next_run_calls: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let mock_scan_repository = MockScanRepository {
            due_scans: expected_due_scans,
            fetch_due_scans_calls: fetch_due_scans_calls.clone(),
            update_next_run_calls: update_next_run_calls.clone(),
        };
        let mock_scan_publisher = MockScanPublisher {
            publish_calls: publish_calls.clone(),
        };
        let mock_utils = MockUtils {
            return_value: expected_next_run,
            calculate_next_run_calls: calculate_next_run_calls.clone(),
        };
        let scheduler = Scheduler::new(
            mock_scan_repository,
            mock_scan_publisher,
            mock_utils,
            expected_poll_interval_secs,
        );
        // Act
        let actual_result = scheduler.poll().await.unwrap();

        // Assert
        assert_eq!(actual_result, ());
        assert_eq!(fetch_due_scans_calls.lock().unwrap().len(), 1);
        assert_eq!(update_next_run_calls.lock().unwrap().len(), 1);
        assert_eq!(
            update_next_run_calls.lock().unwrap()[0],
            (expected_scan_id, expected_next_run)
        );
        assert_eq!(calculate_next_run_calls.lock().unwrap().len(), 1);
        assert_eq!(publish_calls.lock().unwrap().len(), 1);
        assert_eq!(
            publish_calls.lock().unwrap()[0],
            (expected_scan_id, expected_target)
        );
    }
}
