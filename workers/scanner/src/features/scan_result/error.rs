use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanResultError {
    #[error("scan result not found")]
    NotFound,

    #[error("error adding scan result {0}")]
    AddError(#[source] Box<dyn DatabaseError>),

    #[error("internal error")]
    InternalError,
}

impl From<sqlx::Error> for ScanResultError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) => Self::AddError(e),
            _ => Self::InternalError,
        }
    }
}
