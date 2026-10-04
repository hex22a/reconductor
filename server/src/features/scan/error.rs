use reconductor_messaging::error::MqError;
use reconductor_schedule::ScheduleError;
use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("scan not found")]
    NotFound,

    #[error("last cursor not provided")]
    NoLastCursor,

    #[error("error decoding cursor")]
    DecodeError,

    #[error("unable to caluclate schedule")]
    ScheduleError,

    #[error("unable to publish job")]
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
