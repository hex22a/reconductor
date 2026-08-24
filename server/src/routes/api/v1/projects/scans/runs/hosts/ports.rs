use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_PORTS_ENDPOINT_V1,
    features::port::handler::{get_port, list},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    let inner_routes = Router::new()
        .route("/", get(list))
        .route("/{port_id}", get(get_port));

    Router::new().nest(API_PORTS_ENDPOINT_V1, inner_routes)
}
