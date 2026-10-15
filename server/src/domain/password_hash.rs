use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, PartialEq, Newtype)]
pub struct PasswordHash(String);

impl Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PasswordHash").field(&"***").finish()
    }
}
