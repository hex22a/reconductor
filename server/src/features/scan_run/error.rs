use thiserror::Error;

use crate::domain::cursor::CursorError;

#[derive(Debug, Error)]
pub enum ScanRunError {
    #[error("scan run not found")]
    NotFound,

    #[error("last cursor not provided")]
    NoLastCursor,

    #[error("error decoding cursor")]
    DecodeError,
}

impl From<CursorError> for ScanRunError {
    fn from(_: CursorError) -> Self {
        Self::DecodeError
    }
}

impl From<sqlx::Error> for ScanRunError {
    fn from(_: sqlx::Error) -> Self {
        Self::NotFound
    }
}
