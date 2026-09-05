use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::features::scan_result::{error::ScanResultError, model::ScanHostInsert};

pub trait ScanResultRepository {
    fn store_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<ScanHostInsert>,
    ) -> impl Future<Output = Result<(), ScanResultError>> + Send;
}

pub struct PgScanResultRepository {
    db: Arc<PgPool>,
}

impl PgScanResultRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl ScanResultRepository for PgScanResultRepository {
    async fn store_scan_results(
        &self,
        scan_id: Uuid,
        hosts: Vec<ScanHostInsert>,
    ) -> Result<(), ScanResultError> {
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
