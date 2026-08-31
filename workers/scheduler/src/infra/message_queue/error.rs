use std::fmt;

#[derive(Debug)]
pub enum MqError {
    PublishError,
    BuildError,
}

impl From<lapin::Error> for MqError {
    fn from(_: lapin::Error) -> Self {
        Self::PublishError
    }
}

impl From<serde_json::Error> for MqError {
    fn from(_: serde_json::Error) -> Self {
        Self::PublishError
    }
}

impl fmt::Display for MqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MqError::PublishError => write!(f, "Failed to publish message"),
            MqError::BuildError => write!(f, "Failed to build message queue"),
        }
    }
}
