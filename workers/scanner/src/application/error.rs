use std::{env::VarError, num::ParseIntError};

use quick_xml::DeError;
use reconductor_messaging::error::MqError;
use thiserror::Error;

use crate::{
    features::{scan::error::ScanError, scan_result::error::ScanResultError},
    infra::nmap::error::NmapError,
};

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

    #[error("deserilize error")]
    DeserialzeError,
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

impl From<ScanResultError> for AppError {
    fn from(_: ScanResultError) -> Self {
        Self::InternalError
    }
}

impl From<NmapError> for AppError {
    fn from(_: NmapError) -> Self {
        Self::InternalError
    }
}

impl From<MqError> for AppError {
    fn from(_: MqError) -> Self {
        Self::InternalError
    }
}

impl From<DeError> for AppError {
    fn from(_: DeError) -> Self {
        Self::DeserialzeError
    }
}
