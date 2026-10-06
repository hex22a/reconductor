use sqlx::types::{ipnetwork::IpNetwork, mac_address::MacAddress};
use uuid::Uuid;

#[derive(Clone)]
pub struct HostEntity {
    pub id: Uuid,
    pub scan_run_id: Uuid,
    pub ip: IpNetwork,
    pub mac: Option<MacAddress>,
    pub vendor: Option<String>,
    pub hostname: Option<String>,
    pub os_match: Option<String>,
    pub os_accuracy: Option<i32>,
}
