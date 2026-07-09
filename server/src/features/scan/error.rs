use crate::domain::cursor::CursorError;

#[derive(Debug)]
pub(crate) enum ScanError {
    NotFound,
    NoLastCursor,
    DecodeError,
}

impl From<CursorError> for ScanError {
    fn from(_: CursorError) -> Self {
        ScanError::DecodeError
    }
}

impl From<sqlx::Error> for ScanError {
    fn from(_: sqlx::Error) -> Self {
        ScanError::NotFound
    }
}
