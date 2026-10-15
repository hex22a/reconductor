use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, PartialEq, Newtype)]
pub struct CsrfCookie(String);

impl Debug for CsrfCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsrfCookie").field(&"***").finish()
    }
}
