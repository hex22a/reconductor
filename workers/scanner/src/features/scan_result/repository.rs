use std::sync::Arc;

use sqlx::{PgPool, QueryBuilder};
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
                let mut query = QueryBuilder::new(
                    "INSERT INTO recon.scan_ports \
                        (host_id, port, protocol, state, service, product, version, cpes)",
                );

                query.push_values(&host.ports, |mut row, port| {
                    row.push_bind(host_id)
                        .push_bind(port.port)
                        .push_bind(&port.protocol)
                        .push_bind(&port.state)
                        .push_bind(&port.service)
                        .push_bind(&port.product)
                        .push_bind(&port.version)
                        .push_bind(
                            port.cpes
                                .as_ref()
                                .map(|cpes| cpes.iter().map(|c| c.to_string()).collect::<Vec<_>>()),
                        );
                });

                query.build().execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
