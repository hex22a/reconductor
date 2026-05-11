use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{
    constants::API_ME_ENDPOINT_V1,
    features::{
        csrf::token::TokenFeature,
        session::{auth::AuthFeature, middleware::session_middleware},
        user::{handler::me, login::LoginFeature, register::RegisterFeature},
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
        .route(API_ME_ENDPOINT_V1, get(me))
        .route_layer(middleware::from_fn_with_state(
            state,
            session_middleware::<R, L, T, A>,
        ))
}
