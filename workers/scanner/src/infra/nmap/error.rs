use std::{io, string::FromUtf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NmapError {
    #[error("error running nmap")]
    RunError,
}

impl From<io::Error> for NmapError {
    fn from(_: io::Error) -> Self {
        Self::RunError
    }
}

impl From<FromUtf8Error> for NmapError {
    fn from(_: FromUtf8Error) -> Self {
        Self::RunError
    }
}
