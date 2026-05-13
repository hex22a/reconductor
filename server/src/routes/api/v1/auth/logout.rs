use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_LOGOUT_ENDPOINT_V1,
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{
            handler::logout, login::LoginFeature, logout::LogoutFeature, register::RegisterFeature,
        },
    },
    state::AppState,
};

pub fn routes<R, L, O, T, A, C>(state: Arc<AppState<R, L, O, T, A, C>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_LOGOUT_ENDPOINT_V1, post(logout::<R, L, O, T, A, C>))
        .with_state(state)
}
