use chrono::Utc;
use cron::Schedule;
use time::{OffsetDateTime, error::ComponentRange};

pub enum ScheduleError {
    NoNextRun,
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
