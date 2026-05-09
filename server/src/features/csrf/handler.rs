use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    constants::USER_SESSION_COOKIE_NAME,
    domain::error::ServerError,
    features::{
        csrf::{dto::CsrfResponse, token::TokenFeature},
        user::register::RegisterFeature,
    },
    state::AppState,
};

pub async fn handle<R, T>(
    State(mut state): State<AppState<R, T>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    let session_cookie = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string());
    let csrf_token_pair = state.csrf_feature.get_token(session_cookie).await?;
    Ok((
        StatusCode::OK,
        Json(CsrfResponse {
            csrf_token: csrf_token_pair.token,
        }),
    ))
}
