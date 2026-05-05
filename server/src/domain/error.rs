use std::collections::HashMap;

use crate::infra::password::PasswordServiceError;

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
