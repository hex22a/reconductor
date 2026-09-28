use thiserror::Error;

use crate::features::session::repository::SessionRepositoryError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,

    #[error("internal error")]
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
