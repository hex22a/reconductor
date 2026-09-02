use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::features::scan::{error::ScanError, model::ScanStatus};

pub trait ScanRepository {
    fn update_scan_status(
        &self,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> impl Future<Output = Result<(), ScanError>>;
}

pub struct PgScanRepository {
    db: Arc<PgPool>,
}

impl PgScanRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl ScanRepository for PgScanRepository {
    async fn update_scan_status(&self, scan_id: Uuid, status: ScanStatus) -> Result<(), ScanError> {
        sqlx::query!(
            r#"
            UPDATE recon.scans
            SET status = $1::scan_status
            WHERE id = $2
            "#,
            status.as_str() as &str,
            scan_id,
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }
}
