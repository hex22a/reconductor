use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqError {
    #[error("failed to consume message")]
    ConsumeError,

    #[error("failed to build message queue")]
    BuildError,
}

impl From<lapin::Error> for MqError {
    fn from(_: lapin::Error) -> Self {
        Self::ConsumeError
    }
}
