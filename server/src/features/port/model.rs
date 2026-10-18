use crate::domain::types::{
    CpeString, HostId, PortId, PortNumber, PortProtocol, Product, Service, State, Version,
};

#[derive(Clone)]
pub struct PortEntity {
    pub id: PortId,
    pub host_id: HostId,
    pub port: PortNumber,
    pub protocol: Option<PortProtocol>,
    pub state: Option<State>,
    pub service: Option<Service>,
    pub product: Option<Product>,
    pub version: Option<Version>,
    pub cpes: Option<Vec<CpeString>>,
}
