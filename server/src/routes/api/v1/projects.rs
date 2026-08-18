use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    constants::API_PROJECTS_ENDPOINT_V1,
    features::{
        csrf::middleware::session_based,
        project::handler::{create, get_project, list},
        session::middleware::session_middleware,
    },
    state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router {
    let inner_routes = Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .route("/{id}", get(get_project))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_based,
        ))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_middleware,
        ));

    Router::new()
        .nest(API_PROJECTS_ENDPOINT_V1, inner_routes)
        .with_state(state)
}
