use ipnetwork::IpNetwork;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DueScan {
    pub id: Uuid,
    pub target: IpNetwork,
    pub schedule: Option<String>,
}
