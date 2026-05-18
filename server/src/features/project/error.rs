use crate::domain::cursor::CursorError;

#[derive(Debug)]
pub(crate) enum ProjectError {
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
    fn from(va: sqlx::Error) -> Self {
        ProjectError::NotFound
    }
}
