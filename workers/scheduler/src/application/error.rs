use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

use crate::{features::scan::error::ScanError, infra::message_queue::error::MqError};

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

impl From<lapin::Error> for AppError {
    fn from(_: lapin::Error) -> Self {
        Self::InitializationError
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
