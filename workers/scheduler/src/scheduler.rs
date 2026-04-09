use crate::db::scan::ScanRepository;
use crate::queue::publisher::ScanPublisher;
use chrono::{Utc};
use cron::Schedule;
use sqlx::types::time::OffsetDateTime;
use std::str::FromStr;
use tokio::time::{interval, Duration};
use tracing::{error, info};

pub struct Scheduler<R: ScanRepository, P: ScanPublisher> {
    repository: R,
    publisher: P,
    poll_interval: Duration,
}

impl<R: ScanRepository, P: ScanPublisher> Scheduler<R, P> {
    pub fn new(repository: R, publisher: P, poll_interval_secs: u64) -> Self {
        Self {
            repository,
            publisher,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Scheduler started, polling every {}s", self.poll_interval.as_secs());
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

            match self.publisher.publish(scan.id, &scan.target).await {
                Ok(_) => {
                    info!("Published scan {} for target {}", scan.id, scan.target);
                    match calculate_next_run(schedule) {
                        Ok(next_run) => {
                            info!("Next run: {}", next_run);
                            if let Err(e) = self.repository.update_next_run(scan.id, next_run).await {
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

fn calculate_next_run(schedule: &str) -> anyhow::Result<OffsetDateTime> {
    let schedule_with_seconds = format!("0 {}", schedule);
    let next = Schedule::from_str(&schedule_with_seconds)?
        .upcoming(Utc)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No upcoming runs for schedule: {}", schedule))?;

    Ok(OffsetDateTime::from_unix_timestamp(next.timestamp())?)
}

