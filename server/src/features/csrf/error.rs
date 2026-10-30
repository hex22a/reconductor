use thiserror::Error;

use crate::{
    features::{
        csrf::repository::CsrfRepositoryError, session::repository::SessionRepositoryError,
    },
    infra::csrf::CsrfServiceError,
};

#[derive(Debug, Error)]
pub enum CsrfError {
    #[error("anonymous csrf not created")]
    AnonymousNotCreated,

    #[error("invalid session: {0}")]
    InvalidSession(SessionRepositoryError),

    #[error("error storing csrf: {0}")]
    StorageError(CsrfRepositoryError),
}

impl From<CsrfServiceError> for CsrfError {
    fn from(_: CsrfServiceError) -> Self {
        CsrfError::AnonymousNotCreated
    }
}

impl From<CsrfRepositoryError> for CsrfError {
    fn from(value: CsrfRepositoryError) -> Self {
        Self::StorageError(value)
    }
}
