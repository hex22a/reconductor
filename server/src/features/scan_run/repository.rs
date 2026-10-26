use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use reconductor_core::ScanId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::types::ScanRunId, features::scan_run::model::ScanRunEntity};

#[cfg_attr(test, automock)]
pub trait ScanRunRepository {
    fn get_scan_run(
        &self,
        scan_run_id: &ScanRunId,
    ) -> impl Future<Output = Result<ScanRunEntity, sqlx::Error>> + Send;
    fn list_scan_runs(
        &self,
        scan_id: &ScanId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<ScanRunEntity>, sqlx::Error>> + Send;
}

pub struct PgScanRunRepository {
    db: Arc<PgPool>,
}

impl PgScanRunRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl ScanRunRepository for PgScanRunRepository {
    async fn get_scan_run(&self, scan_run_id: &ScanRunId) -> Result<ScanRunEntity, sqlx::Error> {
        let scan_run = sqlx::query_as!(
            ScanRunEntity,
            r#"
            SELECT
                id,
                scan_id,
                created_at
            FROM recon.scan_runs
            WHERE id=$1
            LIMIT 1;
            "#,
            scan_run_id.as_ref(),
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(scan_run)
    }

    async fn list_scan_runs(
        &self,
        scan_id: &ScanId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> Result<Vec<ScanRunEntity>, sqlx::Error> {
        match cursor_id {
            Some(cursor) => {
                sqlx::query_as!(
                    ScanRunEntity,
                    r#"
                    SELECT
                        id,
                        scan_id,
                        created_at
                    FROM recon.scan_runs
                    WHERE scan_id=$1 AND id < $2
                    ORDER BY id DESC
                    LIMIT $3;
                    "#,
                    scan_id.as_ref(),
                    cursor,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    ScanRunEntity,
                    r#"
                    SELECT
                        id,
                        scan_id,
                        created_at
                    FROM recon.scan_runs
                    WHERE scan_id=$1
                    ORDER BY id DESC
                    LIMIT $2;
                    "#,
                    scan_id.as_ref(),
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
