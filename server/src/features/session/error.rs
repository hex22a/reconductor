use crate::infra::persistence::kv::session::SessionRepositoryError;

#[derive(Debug)]
pub enum SessionError {
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
