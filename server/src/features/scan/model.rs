use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "scan_status")]
pub enum ScanStatus {
    #[sqlx(rename = "scheduled")]
    Scheduled,
    #[sqlx(rename = "in progress")]
    InProgress,
    #[sqlx(rename = "done")]
    Done,
}

pub struct ScanEntity {
    pub id: Uuid,
    pub project_id: Uuid,
    pub target: IpNetwork,
    pub status: ScanStatus,
    pub schedule: Option<String>,
    pub created_at: OffsetDateTime,
    pub next_run_at: Option<OffsetDateTime>,
}

pub struct ScanInsert {
    pub project_id: Uuid,
    pub target: IpNetwork,
    pub schedule: Option<String>,
    pub next_run_at: OffsetDateTime,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CreateScanInput {
    pub(crate) target: IpNetwork,
    pub(crate) schedule: Option<String>,
}
