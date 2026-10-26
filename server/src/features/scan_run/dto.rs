use reconductor_core::ScanId;
use serde::Serialize;
use time::OffsetDateTime;

use crate::domain::types::ScanRunId;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScanRunDto {
    pub id: ScanRunId,
    pub scan_id: ScanId,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
