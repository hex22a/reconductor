use time::PrimitiveDateTime;
use uuid::Uuid;

pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: PrimitiveDateTime,
}
