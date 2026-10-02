use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("scan not found")]
    NotFound,

    #[error("error updating scan {0}")]
    UpdateError(#[source] Box<dyn DatabaseError>),

    #[error("internal error")]
    InternalError,
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
