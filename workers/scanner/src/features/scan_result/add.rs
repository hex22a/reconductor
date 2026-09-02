use sqlx::types::{ipnetwork::IpNetwork, mac_address::MacAddress};
use uuid::Uuid;

use crate::{
    domain::result::Host,
    features::scan_result::{
        error::ScanResultError,
        model::{ScanHostInsert, ScanPortInsert},
        repository::ScanResultRepository,
    },
};

pub trait AddScanResultFeature {
    fn add_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<Host>,
    ) -> impl Future<Output = Result<(), ScanResultError>>;
}

pub struct AddScanResult<R: ScanResultRepository> {
    repository: R,
}

impl<R: ScanResultRepository> AddScanResult<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn map_host(host: Host) -> ScanHostInsert {
        let ip: Option<IpNetwork> = host
            .addresses
            .iter()
            .find(|a| a.addrtype == "ipv4" || a.addrtype == "ipv6")
            .map(|a| a.addr.clone().parse().unwrap());

        let mac: Option<MacAddress> = host
            .addresses
            .iter()
            .find(|a| a.addrtype == "mac")
            .map(|a| a.addr.clone().parse().unwrap());

        let vendor = host
            .addresses
            .iter()
            .find(|a| a.addrtype == "mac")
            .and_then(|a| a.vendor.clone());

        let hostname = host
            .hostnames
            .and_then(|h| h.hostnames.into_iter().next())
            .map(|h| h.name);

        let (os_match, os_accuracy) = host
            .os
            .and_then(|o| o.matches.into_iter().next())
            .map(|m| (Some(m.name), m.accuracy.parse::<i32>().ok()))
            .unwrap_or((None, None));

        let ports = host
            .ports
            .map(|p| p.ports)
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.state.state == "open")
            .map(|p| ScanPortInsert {
                port: p.portid.parse::<i32>().unwrap_or(0),
                protocol: Some(p.protocol),
                state: Some(p.state.state),
                service: p.service.as_ref().and_then(|s| s.name.clone()),
                product: p.service.as_ref().and_then(|s| s.product.clone()),
                version: p.service.and_then(|s| s.version),
            })
            .collect();

        ScanHostInsert {
            ip,
            mac,
            hostname,
            vendor,
            os_match,
            os_accuracy,
            ports,
        }
    }
}

impl<R: ScanResultRepository> AddScanResultFeature for AddScanResult<R> {
    async fn add_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<Host>,
    ) -> Result<(), ScanResultError> {
        let hosts: Vec<ScanHostInsert> = hosts.into_iter().map(|h| Self::map_host(h)).collect();
        self.repository.store_scan_results(scan_id, hosts).await?;
        Ok(())
    }
}
