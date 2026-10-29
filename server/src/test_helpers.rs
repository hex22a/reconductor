use axum::{body::to_bytes, http::header::SET_COOKIE, response::Response};
use axum_extra::extract::cookie::Cookie;
use serde::de::DeserializeOwned;

pub async fn body_json<T: DeserializeOwned>(response: Response) -> T {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap_or_else(|e| {
        panic!(
            "failed to parse body as {}: {e}\nbody: {}",
            std::any::type_name::<T>(),
            String::from_utf8_lossy(&bytes)
        )
    })
}

#[track_caller]
pub fn set_cookie(response: &Response, name: &str) -> Cookie<'static> {
    response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .map(|v| Cookie::parse(v.to_str().unwrap().to_owned()).unwrap())
        .find(|c| c.name() == name)
        .unwrap_or_else(|| panic!("no Set-Cookie header for `{name}`"))
}
