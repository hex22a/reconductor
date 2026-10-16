use crate::domain::types::{CsrfCookie, CsrfToken};

#[derive(Clone, Debug)]
pub struct CsrfTokenPair {
    pub token: CsrfToken,
    pub cookie_value: CsrfCookie,
}
