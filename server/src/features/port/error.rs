use crate::domain::cursor::CursorError;

#[derive(Debug)]
pub enum PortError {
    NotFound,
    NoLastCursor,
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
