use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_LOGIN_ENDPOINT_V1,
    features::{
        csrf::token::TokenFeature,
        user::{handler::login, login::LoginFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub fn routes<R, L, T>(state: Arc<AppState<R, L, T>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_LOGIN_ENDPOINT_V1, post(login::<R, L, T>))
        .with_state(state)
}
