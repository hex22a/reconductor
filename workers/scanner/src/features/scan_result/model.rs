use sqlx::types::{ipnetwork::IpNetwork, mac_address::MacAddress};

pub struct ScanHostInsert {
    pub ip: Option<IpNetwork>,
    pub mac: Option<MacAddress>,
    pub vendor: Option<String>,
    pub hostname: Option<String>,
    pub os_match: Option<String>,
    pub os_accuracy: Option<i32>,
    pub ports: Vec<ScanPortInsert>,
}

pub struct ScanPortInsert {
    pub port: i32,
    pub protocol: Option<String>,
    pub state: Option<String>,
    pub service: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
}
