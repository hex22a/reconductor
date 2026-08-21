use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PortDto {
    pub id: Uuid,
    pub port: i32,
    pub protocol: Option<String>,
    pub state: Option<String>,
    pub service: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
}
