use thiserror::Error;

use crate::{
    features::{
        csrf::repository::CsrfRepositoryError, session::repository::SessionRepositoryError,
    },
    infra::{csrf::CsrfServiceError, password::PasswordServiceError, random::RngServiceError},
};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("passwords don't match")]
    PasswordMismatch,

    #[error("password service error")]
    PasswordError,

    #[error("internal error")]
    Interntal,

    #[error("error storing user: {0}")]
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

impl From<CsrfRepositoryError> for UserError {
    fn from(value: CsrfRepositoryError) -> Self {
        match value {
            CsrfRepositoryError::StorageError(e) => UserError::StorageError(e.to_string()),
        }
    }
}
