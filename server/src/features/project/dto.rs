use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: PrimitiveDateTime,
}
