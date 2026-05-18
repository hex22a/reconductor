use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    application::error::ServerError,
    constants::USER_SESSION_COOKIE_NAME,
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub(crate) async fn session_middleware<R, L, O, T, A, C>(
    State(state): State<Arc<AppState<R, L, O, T, A, C>>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let session_token = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value())
        .ok_or(ServerError::Unauthorized)?;
    let user_session = state.auth_feature.auth(session_token).await?;
    req.extensions_mut().insert(user_session);
    Ok(next.run(req).await)
}
