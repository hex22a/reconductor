use serde::Serialize;

use crate::domain::types::{PortId, PortNumber, PortProtocol, Product, Service, State, Version};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PortDto {
    pub id: PortId,
    pub port: PortNumber,
    pub protocol: Option<PortProtocol>,
    pub state: Option<State>,
    pub service: Option<Service>,
    pub product: Option<Product>,
    pub version: Option<Version>,
}
