use axum::{Router, routing::get};

use crate::{
    constants::API_CSRF_ENDPOINT_V1,
    features::{
        csrf::{handler::csrf, token::TokenFeature},
        user::register::RegisterFeature,
    },
    state::AppState,
};

pub fn routes<R, T>(state: AppState<R, T>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_CSRF_ENDPOINT_V1, get(csrf::<R, T>))
        .with_state(state)
}
