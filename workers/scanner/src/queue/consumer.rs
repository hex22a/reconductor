use crate::db::scan::{ScanRepository, ScanStatus, ScanHostInsert, ScanPortInsert};
use crate::nmap;
use crate::nmap::parser::Host;
use futures_lite::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    Channel,
};
use serde::Deserialize;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::types::mac_address::MacAddress;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScanMessage {
    scan_id: Uuid,
    target: String,
}

pub async fn run<R: ScanRepository>(
    repository: R,
    channel: Channel,
) -> anyhow::Result<()> {
    channel
        .queue_declare(
            "scans".into(),
            QueueDeclareOptions { durable: true, ..Default::default() },
            FieldTable::default(),
        )
        .await?;

    // one message at a time per worker
    channel.basic_qos(1, BasicQosOptions::default()).await?;

    let mut consumer = channel
        .basic_consume(
            "scans".into(),
            "scanner_worker".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Scanner consumer started, waiting for messages");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;

        let msg: ScanMessage = match serde_json::from_slice(&delivery.data) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to deserialize message: {}", e);
                delivery.nack(BasicNackOptions { requeue: false, ..Default::default() }).await?;
                continue;
            }
        };

        info!("Received scan job {} for target {}", msg.scan_id, msg.target);

        match process(&repository, msg.scan_id, &msg.target).await {
            Ok(_) => {
                info!("Scan {} completed", msg.scan_id);
                delivery.ack(BasicAckOptions::default()).await?;
            }
            Err(e) => {
                error!("Scan {} failed: {}", msg.scan_id, e);
                delivery.nack(BasicNackOptions { requeue: true, ..Default::default() }).await?;
            }
        }
    }

    Ok(())
}

async fn process<R: ScanRepository>(
    repository: &R,
    scan_id: Uuid,
    target: &str,
) -> anyhow::Result<()> {
    repository.update_scan_status(scan_id, ScanStatus::InProgress).await?;

    let xml = nmap::run(target).await?;
    let result = nmap::parser::parse(&xml)?;

    let hosts: Vec<ScanHostInsert> = result.hosts
        .into_iter()
        .filter(|h| h.status.state == "up")
        .map(map_host)
        .collect();

    repository.store_scan_results(scan_id, hosts).await?;

    repository.update_scan_status(scan_id, ScanStatus::Done).await?;

    Ok(())
}

fn map_host(host: Host) -> ScanHostInsert {
    let ip: Option<IpNetwork> = host.addresses.iter()
        .find(|a| a.addrtype == "ipv4" || a.addrtype == "ipv6")
        .map(|a| a.addr.clone().parse().unwrap());

    let mac: Option<MacAddress> = host.addresses.iter()
        .find(|a| a.addrtype == "mac")
        .map(|a| a.addr.clone().parse().unwrap());

    let vendor = host.addresses.iter()
        .find(|a| a.addrtype == "mac")
        .and_then(|a| a.vendor.clone());

    let hostname = host.hostnames
        .and_then(|h| h.hostnames.into_iter().next())
        .map(|h| h.name);

    let (os_match, os_accuracy) = host.os
        .and_then(|o| o.matches.into_iter().next())
        .map(|m| (Some(m.name), m.accuracy.parse::<i32>().ok()))
        .unwrap_or((None, None));

    let ports = host.ports
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

    ScanHostInsert { ip, mac, hostname, vendor, os_match, os_accuracy, ports }
}
