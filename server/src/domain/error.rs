use std::collections::HashMap;

use crate::{
    features::{csrf::error::CsrfError, user::error::UserError},
    infra::{
        csrf::CsrfServiceError,
        password::PasswordServiceError,
        persistence::kv::{csrf::CsrfRepositoryError, session::SessionError},
    },
};

pub type FieldErrors = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub enum ServerError {
    Internal,
    DatabaseError,
    ValidationError(FieldErrors),
}

impl From<CsrfError> for ServerError {
    fn from(_: CsrfError) -> Self {
        ServerError::Internal
    }
}

impl From<UserError> for ServerError {
    fn from(_: UserError) -> Self {
        ServerError::Internal
    }
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

impl From<CsrfRepositoryError> for ServerError {
    fn from(value: CsrfRepositoryError) -> Self {
        ServerError::Internal
    }
}
