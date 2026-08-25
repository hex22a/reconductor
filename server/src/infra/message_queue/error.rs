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
