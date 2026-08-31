use core::fmt;

use chrono::Utc;
use cron::Schedule;
use time::{OffsetDateTime, error::ComponentRange};

pub enum ScheduleError {
    NoNextRun,
    ConvertionError,
}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleError::NoNextRun => write!(f, "No next run"),
            ScheduleError::ConvertionError => write!(f, "Failed to convert"),
        }
    }
}

impl From<ComponentRange> for ScheduleError {
    fn from(_: ComponentRange) -> Self {
        Self::ConvertionError
    }
}

pub trait SchedulerService {
    fn calculate_next_run(&self, schedule: &Schedule) -> Result<OffsetDateTime, ScheduleError>;
}

pub struct Scheduler;

impl SchedulerService for Scheduler {
    fn calculate_next_run(&self, schedule: &Schedule) -> Result<OffsetDateTime, ScheduleError> {
        let next = schedule
            .upcoming(Utc)
            .next()
            .ok_or(ScheduleError::NoNextRun)?;

        Ok(OffsetDateTime::from_unix_timestamp(next.timestamp())?)
    }
}
