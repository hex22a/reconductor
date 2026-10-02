use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqError {
    #[error("failed to publish message")]
    PublishError,

    #[error("failed to build message queue")]
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
