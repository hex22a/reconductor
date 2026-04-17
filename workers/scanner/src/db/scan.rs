use std::fmt::Display;

use sqlx::{
    PgPool,
    types::{ipnetwork::IpNetwork, mac_address::MacAddress},
};
use uuid::Uuid;

pub enum ScanStatus {
    InProgress,
    Done,
}

impl ScanStatus {
    fn as_str(&self) -> &str {
        match self {
            ScanStatus::InProgress => "in progress",
            ScanStatus::Done => "done",
        }
    }
}

impl Display for ScanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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

pub trait ScanRepository {
    async fn update_scan_status(&self, scan_id: Uuid, status: ScanStatus) -> anyhow::Result<()>;
    async fn store_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<ScanHostInsert>,
    ) -> anyhow::Result<()>;
}

pub struct PgScanRepository {
    pub db: PgPool,
}

impl ScanRepository for PgScanRepository {
    async fn update_scan_status(&self, scan_id: Uuid, status: ScanStatus) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE recon.scans
            SET status = $1::scan_status
            WHERE id = $2
            "#,
            status.as_str() as &str,
            scan_id,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn store_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<ScanHostInsert>,
    ) -> anyhow::Result<()> {
        let mut tx = self.db.begin().await?;

        let scan_run_id: Uuid = sqlx::query_scalar!(
            r#"
            INSERT INTO recon.scan_runs
                (scan_id)
            VALUES
                ($1)
            RETURNING id
            "#,
            scan_id,
        )
        .fetch_one(&mut *tx)
        .await?;

        for host in hosts {
            let host_id: Uuid = sqlx::query_scalar!(
                r#"
                INSERT INTO recon.scan_hosts
                    (scan_run_id, ip, mac, hostname, vendor, os_match, os_accuracy)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id
                "#,
                scan_run_id,
                host.ip,
                host.mac,
                host.hostname,
                host.vendor,
                host.os_match,
                host.os_accuracy,
            )
            .fetch_one(&mut *tx)
            .await?;

            if !host.ports.is_empty() {
                let host_ids = vec![host_id; host.ports.len()];
                let ports: Vec<i32> = host.ports.iter().map(|p| p.port).collect();
                let protocols: Vec<Option<String>> =
                    host.ports.iter().map(|p| p.protocol.clone()).collect();
                let states: Vec<Option<String>> =
                    host.ports.iter().map(|p| p.state.clone()).collect();
                let services: Vec<Option<String>> =
                    host.ports.iter().map(|p| p.service.clone()).collect();
                let products: Vec<Option<String>> =
                    host.ports.iter().map(|p| p.product.clone()).collect();
                let versions: Vec<Option<String>> =
                    host.ports.iter().map(|p| p.version.clone()).collect();
                sqlx::query!(
                    r#"
                    INSERT INTO recon.scan_ports
                        (host_id, port, protocol, state, service, product, version)
                    SELECT * FROM UNNEST(
                        $1::uuid[],
                        $2::int[],
                        $3::text[],
                        $4::text[],
                        $5::text[],
                        $6::text[],
                        $7::text[]
                    )
                    "#,
                    &host_ids as &[Uuid],
                    &ports as &[i32],
                    &protocols as &[Option<String>],
                    &states as &[Option<String>],
                    &services as &[Option<String>],
                    &products as &[Option<String>],
                    &versions as &[Option<String>],
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
