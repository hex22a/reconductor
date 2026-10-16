use sqlx::types::Uuid;
use time::OffsetDateTime;

use crate::domain::types::{CsrfCookie, CsrfToken, Password, PasswordHash, SessionToken, Username};

#[derive(Debug, PartialEq)]
pub struct UserInput {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, PartialEq)]
pub struct AuthSession {
    pub session_id: SessionToken,
    pub csrf_token: CsrfToken,
    pub csrf_cookie: CsrfCookie,
}

#[derive(Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub password_version: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_login_at: OffsetDateTime,
    pub is_active: bool,
}

pub struct UserInsert {
    pub username: Username,
    pub password_hash: PasswordHash,
}
