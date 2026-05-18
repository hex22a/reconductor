use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    constants::API_LOGIN_ENDPOINT_V1,
    features::{csrf::middleware::double_submit, user::handler::login},
    state::AppState,
};

pub(crate) fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_LOGIN_ENDPOINT_V1, post(login))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            double_submit,
        ))
        .with_state(state)
}
