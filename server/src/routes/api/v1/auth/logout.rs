use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{constants::API_LOGOUT_ENDPOINT_V1, features::user::handler::logout, state::AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_LOGOUT_ENDPOINT_V1, post(logout))
        .with_state(state)
}
