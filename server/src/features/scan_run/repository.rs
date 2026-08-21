use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::features::scan_run::model::ScanRunEntity;

pub trait ScanRunRepository {
    fn get_scan_run(
        &self,
        scan_run_id: &Uuid,
    ) -> impl Future<Output = Result<ScanRunEntity, sqlx::Error>> + Send;
    fn list_scan_runs(
        &self,
        scan_id: &Uuid,
        cursor_id: Option<&Uuid>,
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
    async fn get_scan_run(&self, scan_run_id: &Uuid) -> Result<ScanRunEntity, sqlx::Error> {
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
            scan_run_id,
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(scan_run)
    }

    async fn list_scan_runs(
        &self,
        scan_id: &Uuid,
        cursor_id: Option<&Uuid>,
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
                    scan_id,
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
                    scan_id,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
