use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use sqlx::types::time::OffsetDateTime;

pub trait Utils {
    fn calculate_next_run(&self, schedule: &str) -> anyhow::Result<OffsetDateTime>;
}

pub struct SchedulerUtils;

impl Utils for SchedulerUtils {
    fn calculate_next_run(&self, schedule: &str) -> anyhow::Result<OffsetDateTime> {
        let schedule_with_seconds = format!("0 {}", schedule);
        let next = Schedule::from_str(&schedule_with_seconds)?
            .upcoming(Utc)
            .next()
            .ok_or_else(|| anyhow::anyhow!("No upcoming runs for schedule: {}", schedule))?;

        Ok(OffsetDateTime::from_unix_timestamp(next.timestamp())?)
    }
}
