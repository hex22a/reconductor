use time::OffsetDateTime;
use uuid::Uuid;

pub struct ScanRunEntity {
    pub id: Uuid,
    pub scan_id: Uuid,
    pub created_at: OffsetDateTime,
}
