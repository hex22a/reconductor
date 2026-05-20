use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Clone, PartialEq, Debug)]
pub struct ProjectEntity {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
}

pub struct ProjectInsert {
    pub owner_id: Uuid,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CreateProjectInput {
    pub(crate) name: String,
}
