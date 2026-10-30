use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("project not found")]
    NotFound,

    #[error("error storing project: {0}")]
    Storage(sqlx::Error),

    #[error("last cursor not provided")]
    NoLastCursor,

    #[error("error decoding cursor")]
    DecodeError,
}

impl From<CursorError> for ProjectError {
    fn from(_: CursorError) -> Self {
        ProjectError::DecodeError
    }
}

impl From<sqlx::Error> for ProjectError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::Storage(value),
        }
    }
}
