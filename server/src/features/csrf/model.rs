use crate::domain::{csrf_cookie::CsrfCookie, csrf_token::CsrfToken};

#[derive(Clone, Debug, PartialEq)]
pub struct CsrfTokenPair {
    pub token: CsrfToken,
    pub cookie_value: CsrfCookie,
}
