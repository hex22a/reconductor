use std::net::IpAddr;

use mac_address::MacAddress;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostDto {
    pub id: Uuid,
    pub ip: IpAddr,
    pub mac: Option<MacAddress>,
    pub vendor: Option<String>,
    pub hostname: Option<String>,
    pub os_match: Option<String>,
    pub os_accuracy: Option<String>,
}
