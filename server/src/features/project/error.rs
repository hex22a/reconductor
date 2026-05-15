use crate::infra::cursor::CursorError;

#[derive(Debug)]
pub enum ProjectError {
    NotFound,
    NoLastCursor,
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
