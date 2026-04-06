use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NmapRun {
    #[serde(rename = "host", default)]
    pub hosts: Vec<Host>,
}

#[derive(Debug, Deserialize)]
pub struct Host {
    pub status: Status,
    #[serde(rename = "address", default)]
    pub addresses: Vec<Address>,
    #[serde(rename = "hostnames")]
    pub hostnames: Option<Hostnames>,
    pub ports: Option<Ports>,
    pub os: Option<Os>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    #[serde(rename = "@addr")]
    pub addr: String,
    #[serde(rename = "@addrtype")]
    pub addrtype: String,
    #[serde(rename = "@vendor")]
    pub vendor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Hostnames {
    #[serde(rename = "hostname", default)]
    pub hostnames: Vec<Hostname>,
}

#[derive(Debug, Deserialize)]
pub struct Hostname {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Ports {
    #[serde(rename = "port", default)]
    pub ports: Vec<Port>,
}

#[derive(Debug, Deserialize)]
pub struct Port {
    #[serde(rename = "@portid")]
    pub portid: String,
    #[serde(rename = "@protocol")]
    pub protocol: String,
    pub state: PortState,
    pub service: Option<Service>,
}

#[derive(Debug, Deserialize)]
pub struct PortState {
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@product")]
    pub product: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Os {
    #[serde(rename = "osmatch", default)]
    pub matches: Vec<OsMatch>,
}

#[derive(Debug, Deserialize)]
pub struct OsMatch {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@accuracy")]
    pub accuracy: String,
}

pub fn parse(xml: &str) -> anyhow::Result<NmapRun> {
    Ok(from_str(xml)?)
}
