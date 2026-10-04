use chrono::Utc;
use cron::Schedule;
use thiserror::Error;
use time::{OffsetDateTime, error::ComponentRange};

#[derive(Debug, Clone, Error)]
pub enum ScheduleError {
    #[error("no next run")]
    NoNextRun,

    #[error("error conferting schedule")]
    ConvertionError,
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
