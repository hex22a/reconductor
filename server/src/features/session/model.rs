use std::fmt::Debug;

use uuid::Uuid;

use crate::features::{
    csrf::model::{CsrfCookie, CsrfToken},
    user::model::Username,
};

#[derive(Clone, PartialEq)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for SessionToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for SessionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SessionToken").field(&"***").finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub token: SessionToken,
    pub user_id: Uuid,
    pub username: Username,
    pub csrf_token: CsrfToken,
    pub csrf_cookie: CsrfCookie,
}
