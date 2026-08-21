use crate::domain::cursor::CursorError;

#[derive(Debug)]
pub enum ScanRunError {
    NotFound,
    NoLastCursor,
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
