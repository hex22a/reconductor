#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CsrfTokenPair {
    pub(crate) token: String,
    pub(crate) cookie_value: String,
}
