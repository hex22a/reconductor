use serde::Serialize;

#[derive(Serialize)]
struct CsrfResponse {
    csrf_token: String,
}
