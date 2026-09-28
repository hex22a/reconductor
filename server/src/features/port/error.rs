use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum PortError {
    #[error("port not found")]
    NotFound,

    #[error("last cursor not provided")]
    NoLastCursor,

    #[error("error decoding cursor")]
    DecodeError,
}

impl From<CursorError> for PortError {
    fn from(_: CursorError) -> Self {
        Self::DecodeError
    }
}

impl From<sqlx::Error> for PortError {
    fn from(_: sqlx::Error) -> Self {
        Self::NotFound
    }
}
