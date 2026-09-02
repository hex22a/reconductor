use std::fmt;

#[derive(Debug)]
pub enum MqError {
    ConsumeError,
    BuildError,
}

impl From<lapin::Error> for MqError {
    fn from(_: lapin::Error) -> Self {
        Self::ConsumeError
    }
}

impl fmt::Display for MqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MqError::ConsumeError => write!(f, "Failed to consume message"),
            MqError::BuildError => write!(f, "Failed to build message queue"),
        }
    }
}
