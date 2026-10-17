use sqlx::types::{ipnetwork::IpNetwork, mac_address::MacAddress};

use crate::domain::types::{HostId, Hostname, OsAccuracy, OsMatch, ScanRunId, Vendor};

#[derive(Clone)]
pub struct HostEntity {
    pub id: HostId,
    pub scan_run_id: ScanRunId,
    pub ip: IpNetwork,
    pub mac: Option<MacAddress>,
    pub vendor: Option<Vendor>,
    pub hostname: Option<Hostname>,
    pub os_match: Option<OsMatch>,
    pub os_accuracy: Option<OsAccuracy>,
}
