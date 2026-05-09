use std::collections::HashMap;

use crate::infra::{
    csrf::CsrfServiceError,
    password::PasswordServiceError,
    persistence::kv::{csrf::CsrfError, session::SessionError},
};

pub type FieldErrors = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub enum ServerError {
    Internal,
    DatabaseError,
    ValidationError(FieldErrors),
}

impl From<PasswordServiceError> for ServerError {
    fn from(_: PasswordServiceError) -> Self {
        ServerError::Internal
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(value: sqlx::Error) -> Self {
        ServerError::DatabaseError
    }
}

impl From<SessionError> for ServerError {
    fn from(value: SessionError) -> Self {
        ServerError::DatabaseError
    }
}
impl From<CsrfServiceError> for ServerError {
    fn from(value: CsrfServiceError) -> Self {
        ServerError::Internal
    }
}

impl From<CsrfError> for ServerError {
    fn from(value: CsrfError) -> Self {
        ServerError::Internal
    }
}
