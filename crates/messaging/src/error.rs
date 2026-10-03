use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqError {
    #[error("failed to connect to mq server")]
    Connect,

    #[error("failed to create channel")]
    Create,

    #[error("failed to declare message queue")]
    Declare,

    #[error("failed to consume message")]
    Consume,

    #[error("failed to publish message")]
    Publish,

    #[error("failed to ack")]
    Ack,

    #[error("failed to nack")]
    Nack,
}

impl From<serde_json::Error> for MqError {
    fn from(_: serde_json::Error) -> Self {
        Self::Publish
    }
}
