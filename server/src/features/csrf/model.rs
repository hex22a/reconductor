use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct CsrfToken(String);

impl CsrfToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for CsrfToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for CsrfToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfToken").field(&"***").finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct CsrfCookie(String);

impl CsrfCookie {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for CsrfCookie {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for CsrfCookie {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for CsrfCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfCookie").field(&"***").finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CsrfTokenPair {
    pub token: CsrfToken,
    pub cookie_value: CsrfCookie,
}
