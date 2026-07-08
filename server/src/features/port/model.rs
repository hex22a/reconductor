use uuid::Uuid;

pub struct PortEntity {
    id: Uuid,
    host_id: Uuid,
    port: i32,
    protocol: Option<String>,
    state: Option<String>,
    service: Option<String>,
    product: Option<String>,
    version: Option<String>,
}
