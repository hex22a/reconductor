use sqlx::types::{Uuid, time::PrimitiveDateTime};

#[derive(Clone)]
pub struct ProjectEntity {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: PrimitiveDateTime,
}

pub struct ProjectInsert {
    pub owner_id: Uuid,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CreateProjectInput {
    pub(crate) name: String,
}
