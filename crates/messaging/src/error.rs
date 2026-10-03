use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqError {
    #[error("failed to consume message")]
    Consume,

    #[error("failed to publish message")]
    Publish,

    #[error("failed to ack")]
    Ack,

    #[error("failed to nack")]
    Nack,

    #[error("failed to build message queue")]
    Build,
}

impl From<serde_json::Error> for MqError {
    fn from(_: serde_json::Error) -> Self {
        Self::Publish
    }
}
