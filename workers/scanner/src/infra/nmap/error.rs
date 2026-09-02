use std::{io, string::FromUtf8Error};

#[derive(Debug)]
pub enum NmapError {
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
