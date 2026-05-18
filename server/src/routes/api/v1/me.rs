use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{
    constants::API_ME_ENDPOINT_V1,
    features::{session::middleware::session_middleware, user::handler::me},
    state::AppState,
};

pub(crate) fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_ME_ENDPOINT_V1, get(me))
        .route_layer(middleware::from_fn_with_state(state, session_middleware))
}
