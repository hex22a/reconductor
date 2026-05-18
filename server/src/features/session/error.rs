use crate::features::session::repository::SessionRepositoryError;

#[derive(Debug)]
pub(crate) enum SessionError {
    NotFound,
    Internal,
}

impl From<SessionRepositoryError> for SessionError {
    fn from(value: SessionRepositoryError) -> Self {
        match value {
            SessionRepositoryError::NotFound => SessionError::NotFound,
            _ => SessionError::Internal,
        }
    }
}
