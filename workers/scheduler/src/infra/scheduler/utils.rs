use chrono::Utc;
use cron::Schedule;
use sqlx::types::time::OffsetDateTime;
use std::str::FromStr;

pub trait Utils {
    fn calculate_next_run(&self, schedule: &str) -> anyhow::Result<OffsetDateTime>;
}

pub struct SchedulerUtils;

impl Utils for SchedulerUtils {
    fn calculate_next_run(&self, schedule: &str) -> anyhow::Result<OffsetDateTime> {
        let next = Schedule::from_str(schedule)
            .expect("Error parsing schedule")
            .upcoming(Utc)
            .next()
            .ok_or_else(|| anyhow::anyhow!("No upcoming runs for schedule: {}", schedule))?;

        Ok(OffsetDateTime::from_unix_timestamp(next.timestamp())?)
    }
}
