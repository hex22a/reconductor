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

mod scans;

pub fn routes(state: Arc<AppState>) -> Router {
    let scan_routes = scans::routes();
    let project_routes = Router::new()
        .route("/{project_id}", get(get_project))
        .nest("/{project_id}", scan_routes);
    let inner_routes = Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .merge(project_routes)
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
