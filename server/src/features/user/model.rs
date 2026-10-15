use sqlx::types::Uuid;
use time::OffsetDateTime;

use crate::domain::{
    csrf_cookie::CsrfCookie, csrf_token::CsrfToken, password::Password,
    password_hash::PasswordHash, session_token::SessionToken, username::Username,
};

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
