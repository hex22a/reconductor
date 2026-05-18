use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    application::error::ServerError, constants::USER_SESSION_COOKIE_NAME, state::AppState,
};

pub(crate) async fn session_middleware(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError> {
    let session_token = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value())
        .ok_or(ServerError::Unauthorized)?;
    let user_session = state.auth_feature.auth(session_token).await?;
    req.extensions_mut().insert(user_session);
    Ok(next.run(req).await)
}
