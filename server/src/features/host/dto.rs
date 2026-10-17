use std::net::IpAddr;

use mac_address::MacAddress;
use serde::Serialize;

use crate::domain::types::{HostId, Hostname, OsAccuracy, OsMatch, Vendor};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostDto {
    pub id: HostId,
    pub ip: IpAddr,
    pub mac: Option<MacAddress>,
    pub vendor: Option<Vendor>,
    pub hostname: Option<Hostname>,
    pub os_match: Option<OsMatch>,
    pub os_accuracy: Option<OsAccuracy>,
}
