use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    constants::API_PROJECTS_ENDPOINT_V1,
    features::{
        csrf::middleware::session_based,
        project::handler::{create, list},
        session::middleware::session_middleware,
    },
    state::AppState,
};

pub(crate) fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_PROJECTS_ENDPOINT_V1, post(create))
        .route(API_PROJECTS_ENDPOINT_V1, get(list))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_based,
        ))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_middleware,
        ))
        .with_state(state)
}
