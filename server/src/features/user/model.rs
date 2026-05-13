use sqlx::types::{Uuid, time::PrimitiveDateTime};

#[derive(Debug, PartialEq)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq)]
pub struct AuthSession {
    pub session_id: String,
    pub csrf_token: String,
}

#[derive(Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub password_version: i16,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub last_login_at: PrimitiveDateTime,
    pub is_active: bool,
}

pub struct UserInsert {
    pub username: String,
    pub password_hash: String,
}
