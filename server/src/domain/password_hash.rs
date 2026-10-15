use std::fmt::Debug;

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
