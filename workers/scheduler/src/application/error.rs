use std::{env::VarError, num::ParseIntError};

use reconductor_messaging::error::MqError;
use thiserror::Error;

use crate::features::scan::error::ScanError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("environment error {0}")]
    EnvironmentError(String),

    #[error("error initialization")]
    InitializationError,

    #[error("error parsing int")]
    ParseIntError,

    #[error("internal error")]
    InternalError,
}

impl From<VarError> for AppError {
    fn from(value: VarError) -> Self {
        Self::EnvironmentError(value.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<ScanError> for AppError {
    fn from(_: ScanError) -> Self {
        Self::InternalError
    }
}

impl From<MqError> for AppError {
    fn from(_: MqError) -> Self {
        Self::InternalError
    }
}
