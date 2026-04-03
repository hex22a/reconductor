use uuid::Uuid;
use sqlx::{PgPool, types::ipnetwork::IpNetwork};

#[derive(Debug, sqlx::FromRow)]
pub struct DueScan {
    pub id: Uuid,
    pub target: IpNetwork,
    pub schedule: Option<String>,
}

pub trait ScanRepository {
    async fn fetch_due_scans(&self) -> anyhow::Result<Vec<DueScan>>;
}

pub struct Repository {
    db: PgPool,
}

impl ScanRepository for Repository {
    async fn fetch_due_scans(&self) -> anyhow::Result<Vec<DueScan>> {
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
}


