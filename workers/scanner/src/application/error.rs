use std::{env::VarError, fmt, num::ParseIntError};

use quick_xml::DeError;

use crate::{
    features::{scan::error::ScanError, scan_result::error::ScanResultError},
    infra::{message_queue::error::MqError, nmap::error::NmapError},
};

#[derive(Debug)]
pub enum AppError {
    EnvironmentError(String),
    InitializationError,
    ParseIntError,
    InternalError,
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

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scanner error")
    }
}
