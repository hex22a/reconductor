use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum HostError {
    #[error("host not found")]
    NotFound,

    #[error("last cursor not provided")]
    NoLastCursor,

    #[error("error decoding cursor")]
    DecodeError,
}

impl From<CursorError> for HostError {
    fn from(_: CursorError) -> Self {
        Self::DecodeError
    }
}

impl From<sqlx::Error> for HostError {
    fn from(_: sqlx::Error) -> Self {
        Self::NotFound
    }
}
