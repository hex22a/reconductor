use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
}
