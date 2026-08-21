use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct ScanRunEntity {
    pub id: Uuid,
    pub scan_id: Uuid,
    pub created_at: OffsetDateTime,
}
