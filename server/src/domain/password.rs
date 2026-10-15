use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, PartialEq, Newtype)]
pub struct Password(String);

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").field(&"***").finish()
    }
}
