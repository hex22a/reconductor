use std::fmt::Display;

use uuid::Uuid;
use sqlx::{PgPool, types::{ipnetwork::IpNetwork}};

pub enum ScanStatus {
    InProgress,
    Done,
}

impl ScanStatus {
    fn as_str(&self) -> &str{
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
    pub ip: Option<String>,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub hostname: Option<String>,
    pub os_match: Option<String>,
    pub os_accuracy: Option<u32>,
    pub ports: Vec<ScanPortInsert>,
}

pub struct ScanPortInsert {
    pub port: u32,
    pub protocol: Option<String>,
    pub state: Option<String>,
    pub service: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
}

pub trait ScanRepository {
    async fn update_scan_status(
        &self,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> anyhow::Result<()>;
    async fn add_scan_host(
        &self,
        host_insert: ScanHostInsert,
    ) -> anyhow::Result<()>;
    async fn add_port_insert(
        &self,
        port_insert: ScanPortInsert,
    ) -> anyhow::Result<()>;
}

pub struct PgScanRepository {
    pub db: PgPool,
}

impl ScanRepository for PgScanRepository {
    async fn update_scan_status(
        &self,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> anyhow::Result<()> {
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

    async fn add_scan_host(
        &self,
        host_insert: ScanHostInsert,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn add_port_insert(
        &self,
        port_insert: ScanPortInsert,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

