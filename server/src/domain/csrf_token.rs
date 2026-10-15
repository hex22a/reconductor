use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, PartialEq, Newtype)]
pub struct CsrfToken(String);

impl Debug for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfToken").field(&"***").finish()
    }
}
