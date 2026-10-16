use std::fmt::Debug;

use newtype_derive::Newtype;
use uuid::Uuid;

#[derive(Clone, PartialEq, Newtype)]
pub struct CsrfCookie(String);

impl Debug for CsrfCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfCookie").field(&"***").finish()
    }
}

#[derive(Clone, PartialEq, Newtype)]
pub struct CsrfToken(String);

impl Debug for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfToken").field(&"***").finish()
    }
}

#[derive(Clone, Debug, PartialEq, Newtype)]
pub struct Hostname(String);

#[derive(Clone, PartialEq, Newtype)]
pub struct Password(String);

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").field(&"***").finish()
    }
}

#[derive(Clone, PartialEq, Newtype)]
pub struct PasswordHash(String);

impl Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PasswordHash").field(&"***").finish()
    }
}

#[derive(Clone, PartialEq, Newtype)]
pub struct SessionToken(String);

impl Debug for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SessionToken").field(&"***").finish()
    }
}

#[derive(Clone, Debug, PartialEq, Newtype)]
pub struct Username(String);

#[derive(Clone, Debug, PartialEq, Newtype)]
pub struct Vendor(String);

pub struct UserId(Uuid);

impl UserId {
    pub fn new(value: impl Into<Uuid>) -> Self {
        Self(value.into())
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
