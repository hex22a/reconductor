use std::fmt;

use crate::infra::{csrf::CsrfServiceError, persistence::kv::csrf::CsrfRepositoryError};

#[derive(Debug)]
pub enum CsrfError {
    AnonymousNotCreated,
    StorageError(String),
}

impl From<CsrfServiceError> for CsrfError {
    fn from(_: CsrfServiceError) -> Self {
        CsrfError::AnonymousNotCreated
    }
}

impl From<CsrfRepositoryError> for CsrfError {
    fn from(value: CsrfRepositoryError) -> Self {
        match value {
            CsrfRepositoryError::StorageError(e) => CsrfError::StorageError(e.to_string()),
        }
    }
}

impl fmt::Display for CsrfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsrfError::StorageError(e) => write!(f, "error storing csrf: {}", e),
            CsrfError::AnonymousNotCreated => write!(f, "anonymous csrf not created"),
        }
    }
}
