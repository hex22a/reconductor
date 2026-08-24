use std::collections::HashMap;

use crate::{
    features::{
        csrf::error::CsrfError, host::error::HostError, port::error::PortError,
        project::error::ProjectError, scan::error::ScanError, scan_run::error::ScanRunError,
        session::error::SessionError, user::error::UserError,
    },
    infra::password::PasswordServiceError,
};

pub type FieldErrors = HashMap<String, Vec<String>>;

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

impl From<ProjectError> for ServerError {
    fn from(_: ProjectError) -> Self {
        ServerError::Internal
    }
}

impl From<ScanError> for ServerError {
    fn from(_: ScanError) -> Self {
        Self::Internal
    }
}

impl From<ScanRunError> for ServerError {
    fn from(_: ScanRunError) -> Self {
        Self::Internal
    }
}

impl From<HostError> for ServerError {
    fn from(_: HostError) -> Self {
        Self::Internal
    }
}

impl From<PortError> for ServerError {
    fn from(_: PortError) -> Self {
        Self::Internal
    }
}
