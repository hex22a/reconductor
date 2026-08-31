use std::{env::VarError, num::ParseIntError};

#[derive(Debug)]
pub enum AppError {
    EnvironmentError(String),
    InitializationError,
    ParseIntError,
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

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        Self::InternalError
    }
}
