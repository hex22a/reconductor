use thiserror::Error;

use crate::{features::csrf::repository::CsrfRepositoryError, infra::csrf::CsrfServiceError};

#[derive(Debug, Error)]
pub enum CsrfError {
    #[error("anonymous csrf not created")]
    AnonymousNotCreated,

    #[error("error storing csrf: {0}")]
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
