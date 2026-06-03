use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

pub enum ScanStatus {
    Scheduled(String),
    InProgress(String),
    Done(String),
}

pub struct ScanEntity {
    pub id: Uuid,
    pub project_id: Uuid,
    pub target: IpNetwork,
    pub status: ScanStatus,
    pub schedule: Option<String>,
    pub created_at: OffsetDateTime,
    pub next_run_at: OffsetDateTime,
}

pub struct ScanInsert {
    pub project_id: Uuid,
    pub target: String,
    pub schedule: Option<String>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CreateScanInput {
    pub(crate) target: IpNetwork,
    pub(crate) schedule: Option<String>,
}
