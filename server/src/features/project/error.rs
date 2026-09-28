use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("project not found")]
    NotFound,

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
    fn from(_: sqlx::Error) -> Self {
        ProjectError::NotFound
    }
}
