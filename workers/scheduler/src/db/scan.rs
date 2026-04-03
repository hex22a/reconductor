use uuid::Uuid;
use sqlx::{PgPool, types::ipnetwork::IpNetwork};

#[derive(Debug, sqlx::FromRow)]
pub struct DueScan {
    pub id: Uuid,
    pub target: IpNetwork,
    pub schedule: Option<String>,
}

pub async fn fetch_due_scans(db: &PgPool) -> anyhow::Result<Vec<DueScan>> {
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
    .fetch_all(db)
    .await?;

    Ok(scans)
}
