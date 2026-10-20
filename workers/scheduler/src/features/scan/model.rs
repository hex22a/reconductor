use ipnetwork::IpNetwork;
use reconductor_core::ScanId;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DueScan {
    pub id: ScanId,
    pub target: IpNetwork,
    pub schedule: Option<String>,
}
