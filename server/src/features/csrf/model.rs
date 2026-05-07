#[derive(Debug, PartialEq)]
pub struct CsrfTokenPair {
    pub token: String,
    pub cookie_value: Option<String>,
}
