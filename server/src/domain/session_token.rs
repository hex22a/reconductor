use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, PartialEq, Newtype)]
pub struct SessionToken(String);

impl Debug for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SessionToken").field(&"***").finish()
    }
}
