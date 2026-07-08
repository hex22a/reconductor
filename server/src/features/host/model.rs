use sqlx::types::{ipnetwork::IpNetwork, mac_address::MacAddress};
use uuid::Uuid;

pub struct HostEntity {
    id: Uuid,
    scan_run_id: Uuid,
    ip: IpNetwork,
    mac: Option<MacAddress>,
    vendor: Option<String>,
    hostname: Option<String>,
    os_match: Option<String>,
    os_accuracy: Option<i32>,
}
