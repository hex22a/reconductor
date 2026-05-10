use core::fmt;

use crate::infra::password::PasswordServiceError;

#[derive(Debug)]
pub enum UserError {
    PasswordMismatch,
    PasswordError,
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

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::StorageError(e) => write!(f, "error storing user: {}", e),
            UserError::PasswordError => write!(f, "password service error"),
            UserError::PasswordMismatch => write!(f, "password mismatch"),
        }
    }
}
