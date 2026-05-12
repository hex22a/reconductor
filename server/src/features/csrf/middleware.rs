use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    domain::error::ServerError,
    features::{
        csrf::token::TokenFeature,
        session::auth::AuthFeature,
        user::{login::LoginFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub async fn csrf_middleware<R, L, T, A>(
    State(state): State<Arc<AppState<R, L, T, A>>>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
{
    Ok(next.run(req).await)
}
