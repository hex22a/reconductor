use crate::{domain::cursor::CursorError, infra::scheduler::ScheduleError};

#[derive(Debug)]
pub(crate) enum ScanError {
    NotFound,
    NoLastCursor,
    DecodeError,
    ScheduleError,
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
