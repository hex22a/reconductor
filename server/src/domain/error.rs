use std::collections::HashMap;

use crate::{
    features::{csrf::error::CsrfError, user::error::UserError},
    infra::password::PasswordServiceError,
};

pub type FieldErrors = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub enum ServerError {
    Internal,
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
