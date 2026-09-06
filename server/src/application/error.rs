use std::{collections::HashMap, env::VarError, num::ParseIntError};

use axum::http::header::InvalidHeaderValue;
use hex::FromHexError;

use crate::{
    features::{
        csrf::error::CsrfError, host::error::HostError, port::error::PortError,
        project::error::ProjectError, scan::error::ScanError, scan_run::error::ScanRunError,
        session::error::SessionError, user::error::UserError,
    },
    infra::password::PasswordServiceError,
};

#[derive(Debug)]
pub enum AppError {
    EnvironmentError(String),
    HexDecodeError(String),
    CsrfLengthError,
    HeaderError,
    ParseIntError,
    InitializationError,
}

impl From<VarError> for AppError {
    fn from(value: VarError) -> Self {
        Self::EnvironmentError(value.to_string())
    }
}

impl From<FromHexError> for AppError {
    fn from(value: FromHexError) -> Self {
        Self::HexDecodeError(value.to_string())
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::HeaderError
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<lapin::Error> for AppError {
    fn from(_: lapin::Error) -> Self {
        Self::InitializationError
    }
}

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
