use sqlx::{PgPool, types::time::OffsetDateTime};
use uuid::Uuid;

use crate::features::scan::{error::ScanError, model::DueScan};

pub trait ScanRepository {
    fn fetch_due_scans(&self) -> impl Future<Output = Result<Vec<DueScan>, ScanError>> + Send;
    fn update_next_run(
        &self,
        scan_id: Uuid,
        next_run_at: OffsetDateTime,
    ) -> impl Future<Output = Result<(), ScanError>> + Send;
}

pub struct PgScanRepository {
    pub db: PgPool,
}

impl ScanRepository for PgScanRepository {
    async fn fetch_due_scans(&self) -> Result<Vec<DueScan>, ScanError> {
        let scans = sqlx::query_as!(
            DueScan,
            r#"
            SELECT id, target, schedule
            FROM recon.scans
            WHERE schedule IS NOT NULL
                AND next_run_at <= NOW()
                AND status != 'in progress'
            FOR UPDATE SKIP LOCKED
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(scans)
    }
    async fn update_next_run(
        &self,
        scan_id: Uuid,
        next_run_at: OffsetDateTime,
    ) -> Result<(), ScanError> {
        sqlx::query!(
            r#"
            UPDATE recon.scans
            SET next_run_at = $1
            WHERE id = $2
            "#,
            next_run_at,
            scan_id,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
