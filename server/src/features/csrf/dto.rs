use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct CsrfResponse {
    pub(crate) csrf_token: String,
}
