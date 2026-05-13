use core::fmt;

use crate::{
    features::session::repository::SessionRepositoryError,
    infra::{csrf::CsrfServiceError, password::PasswordServiceError, random::RngServiceError},
};

#[derive(Debug)]
pub enum UserError {
    PasswordMismatch,
    PasswordError,
    Interntal,
    StorageError(String),
}

impl From<PasswordServiceError> for UserError {
    fn from(_: PasswordServiceError) -> Self {
        UserError::PasswordError
    }
}

impl From<sqlx::Error> for UserError {
    fn from(value: sqlx::Error) -> Self {
        UserError::StorageError(value.to_string())
    }
}

impl From<RngServiceError> for UserError {
    fn from(_: RngServiceError) -> Self {
        UserError::Interntal
    }
}

impl From<CsrfServiceError> for UserError {
    fn from(_: CsrfServiceError) -> Self {
        UserError::Interntal
    }
}

impl From<SessionRepositoryError> for UserError {
    fn from(value: SessionRepositoryError) -> Self {
        match value {
            SessionRepositoryError::StorageError(e) => UserError::StorageError(e.to_string()),
            _ => UserError::Interntal,
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::StorageError(e) => write!(f, "error storing user: {}", e),
            UserError::PasswordError => write!(f, "password service error"),
            UserError::PasswordMismatch => write!(f, "password mismatch"),
            UserError::Interntal => write!(f, "internal error"),
        }
    }
}
