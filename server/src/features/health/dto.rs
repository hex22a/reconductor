use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct HealthResponse {
    pub(crate) healthy: bool,
}
