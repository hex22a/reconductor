use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    constants::API_SCANS_ENDPOINT_V1,
    features::{
        csrf::middleware::session_based, scan::handler::create,
        session::middleware::session_middleware,
    },
    state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router {
    let inner_routes = Router::new()
        .route("/", post(create))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_based,
        ))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_middleware,
        ));

    Router::new()
        .nest(API_SCANS_ENDPOINT_V1, inner_routes)
        .with_state(state)
}
