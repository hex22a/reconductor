use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_LOGIN_ENDPOINT_V1,
    features::{
        csrf::token::TokenFeature,
        session::auth::AuthFeature,
        user::{handler::login, login::LoginFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub fn routes<R, L, T, A>(state: Arc<AppState<R, L, T, A>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_LOGIN_ENDPOINT_V1, post(login::<R, L, T, A>))
        .with_state(state)
}
