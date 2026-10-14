use std::fmt::Debug;

use sqlx::types::Uuid;
use time::OffsetDateTime;

use crate::features::{
    csrf::model::{CsrfCookie, CsrfToken},
    session::model::SessionToken,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").field(&"***").finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PasswordHash").field(&"***").finish()
    }
}

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
