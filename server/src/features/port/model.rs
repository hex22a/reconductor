use uuid::Uuid;

#[derive(Clone)]
pub struct PortEntity {
    pub id: Uuid,
    pub host_id: Uuid,
    pub port: i32,
    pub protocol: Option<String>,
    pub state: Option<String>,
    pub service: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
    pub cpes: Option<Vec<String>>,
}
