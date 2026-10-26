use crate::domain::types::ScanRunId;
use reconductor_core::ScanId;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct ScanRunEntity {
    pub id: ScanRunId,
    pub scan_id: ScanId,
    pub created_at: OffsetDateTime,
}
