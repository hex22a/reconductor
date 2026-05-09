use serde::Serialize;

#[derive(Serialize)]
pub struct CsrfResponse {
    pub csrf_token: String,
}
