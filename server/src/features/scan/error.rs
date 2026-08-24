use crate::{
    domain::cursor::CursorError,
    infra::{message_queue::error::MqError, scheduler::ScheduleError},
};

#[derive(Debug)]
pub enum ScanError {
    NotFound,
    NoLastCursor,
    DecodeError,
    ScheduleError,
    PublishError,
}

impl From<CursorError> for ScanError {
    fn from(_: CursorError) -> Self {
        ScanError::DecodeError
    }
}

impl From<sqlx::Error> for ScanError {
    fn from(_: sqlx::Error) -> Self {
        ScanError::NotFound
    }
}

impl From<ScheduleError> for ScanError {
    fn from(_: ScheduleError) -> Self {
        Self::ScheduleError
    }
}

impl From<MqError> for ScanError {
    fn from(_: MqError) -> Self {
        Self::PublishError
    }
}
