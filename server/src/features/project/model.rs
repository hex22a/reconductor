use time::OffsetDateTime;

use crate::domain::types::{ProjectId, UserId};

#[derive(Clone, PartialEq, Debug)]
pub struct ProjectEntity {
    pub id: ProjectId,
    pub owner_id: UserId,
    pub name: String,
    pub created_at: OffsetDateTime,
}

pub struct ProjectInsert {
    pub owner_id: UserId,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CreateProjectInput {
    pub name: String,
}
