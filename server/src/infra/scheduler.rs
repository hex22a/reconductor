use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use time::{OffsetDateTime, error::ComponentRange};

pub enum ScheduleError {
    ParseError,
    NoNextRun,
    ConvertionError,
}

impl From<cron::error::Error> for ScheduleError {
    fn from(_: cron::error::Error) -> Self {
        Self::ParseError
    }
}

impl From<ComponentRange> for ScheduleError {
    fn from(_: ComponentRange) -> Self {
        Self::ConvertionError
    }
}

pub trait SchedulerService {
    fn calculate_next_run(&self, schedule: &str) -> Result<OffsetDateTime, ScheduleError>;
}

pub struct Scheduler;

impl SchedulerService for Scheduler {
    fn calculate_next_run(&self, schedule: &str) -> Result<OffsetDateTime, ScheduleError> {
        let schedule_with_seconds = format!("0 {}", schedule);
        let next = Schedule::from_str(&schedule_with_seconds)?
            .upcoming(Utc)
            .next()
            .ok_or(ScheduleError::NoNextRun)?;

        Ok(OffsetDateTime::from_unix_timestamp(next.timestamp())?)
    }
}
