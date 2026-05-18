use std::collections::HashMap;

use crate::{
    features::{csrf::error::CsrfError, session::error::SessionError, user::error::UserError},
    infra::password::PasswordServiceError,
};

pub(crate) type FieldErrors = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub enum ServerError {
    Internal,
    Unauthorized,
    Forbidden,
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

impl From<SessionError> for ServerError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::NotFound => ServerError::Unauthorized,
            SessionError::Internal => ServerError::Internal,
        }
    }
}
