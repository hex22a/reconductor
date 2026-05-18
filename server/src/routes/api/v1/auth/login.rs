use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    constants::API_LOGIN_ENDPOINT_V1,
    features::{
        csrf::{middleware::double_submit, token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{
            handler::login, login::LoginFeature, logout::LogoutFeature, register::RegisterFeature,
        },
    },
    state::AppState,
};

pub(crate) fn routes<R, L, O, T, A, C>(state: Arc<AppState<R, L, O, T, A, C>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_LOGIN_ENDPOINT_V1, post(login::<R, L, O, T, A, C>))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            double_submit::<R, L, O, T, A, C>,
        ))
        .with_state(state)
}
