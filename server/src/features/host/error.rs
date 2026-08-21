use crate::domain::cursor::CursorError;

#[derive(Debug)]
pub enum HostError {
    NotFound,
    NoLastCursor,
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
