use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("scan not found")]
    NotFound,

    #[error("failed to update scan {0}")]
    UpdateError(#[source] Box<dyn DatabaseError>),

    #[error("internal database error")]
    InternalError,

    #[error("failed to parse schedule")]
    ScheduleParsingError,
}

impl From<sqlx::Error> for ScanError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) => Self::UpdateError(e),
            _ => Self::InternalError,
        }
    }
}

impl From<cron::error::Error> for ScanError {
    fn from(_: cron::error::Error) -> Self {
        Self::ScheduleParsingError
    }
}
