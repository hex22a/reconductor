use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::types::{HostId, Hostname, OsAccuracy, OsMatch, ScanRunId, Vendor},
    features::host::model::HostEntity,
};

#[cfg_attr(test, automock)]
pub trait HostRepository {
    fn get_host(
        &self,
        host_id: &HostId,
    ) -> impl Future<Output = Result<HostEntity, sqlx::Error>> + Send;

    fn list_hosts(
        &self,
        scan_run_id: &ScanRunId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<HostEntity>, sqlx::Error>> + Send;
}

pub struct PgHostRepository {
    db: Arc<PgPool>,
}

impl PgHostRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl HostRepository for PgHostRepository {
    async fn get_host(&self, host_id: &HostId) -> Result<HostEntity, sqlx::Error> {
        let host = sqlx::query_as!(
            HostEntity,
            r#"
            SELECT
                id,
                scan_run_id,
                ip,
                mac,
                vendor as "vendor: Vendor",
                hostname as "hostname: Hostname",
                os_match as "os_match: OsMatch",
                os_accuracy as "os_accuracy: OsAccuracy"
            FROM recon.scan_hosts
            WHERE id=$1
            LIMIT 1;
            "#,
            host_id.as_ref(),
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(host)
    }

    async fn list_hosts(
        &self,
        scan_run_id: &ScanRunId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> Result<Vec<HostEntity>, sqlx::Error> {
        match cursor_id {
            Some(cursor) => {
                sqlx::query_as!(
                    HostEntity,
                    r#"
                    SELECT
                        id,
                        scan_run_id,
                        ip,
                        mac,
                        vendor as "vendor: Vendor",
                        hostname as "hostname: Hostname",
                        os_match as "os_match: OsMatch",
                        os_accuracy as "os_accuracy: OsAccuracy"
                    FROM recon.scan_hosts
                    WHERE scan_run_id=$1 and id < $2
                    ORDER BY id DESC
                    LIMIT $3;
                    "#,
                    scan_run_id.as_ref(),
                    cursor,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    HostEntity,
                    r#"
                    SELECT
                        id,
                        scan_run_id,
                        ip,
                        mac,
                        vendor as "vendor: Vendor",
                        hostname as "hostname: Hostname",
                        os_match as "os_match: OsMatch",
                        os_accuracy as "os_accuracy: OsAccuracy"
                    FROM recon.scan_hosts
                    WHERE scan_run_id=$1
                    ORDER BY id DESC
                    LIMIT $2;
                    "#,
                    scan_run_id.as_ref(),
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
