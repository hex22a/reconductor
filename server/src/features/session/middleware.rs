use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    constants::USER_SESSION_COOKIE_NAME,
    domain::error::ServerError,
    features::{
        csrf::token::TokenFeature,
        session::auth::AuthFeature,
        user::{login::LoginFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub async fn session_middleware<R, L, T, A>(
    State(state): State<Arc<AppState<R, L, T, A>>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
{
    let session_token = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::Unauthorized)?;
    let user_session = state.auth_feature.auth(session_token).await?;
    req.extensions_mut().insert(user_session);
    Ok(next.run(req).await)
}
