use std::fmt;

use sqlx::error::DatabaseError;

#[derive(Debug)]
pub enum ScanError {
    NotFound,
    UpdateError(Box<dyn DatabaseError>),
    InternalError,
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

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::NotFound => write!(f, "Scan not found"),
            ScanError::UpdateError(database_error) => {
                write!(f, "Failed to update scan: {}", database_error)
            }
            ScanError::InternalError => write!(f, "Internal database error"),
            ScanError::ScheduleParsingError => write!(f, "Failed to parse schedule"),
        }
    }
}
