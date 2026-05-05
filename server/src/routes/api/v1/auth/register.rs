use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1, features::user::handler::register, state::AppState,
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(register))
        .with_state(state)
}
