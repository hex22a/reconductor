use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1, features::user::handler::register,
    infra::password::PasswordService, persistence::db::user::UserRepository, state::AppState,
};

pub fn routes<P, U>(state: AppState<P, U>) -> Router
where
    P: PasswordService + Clone + Send + Sync + 'static,
    U: UserRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(register::<P, U>))
        .with_state(state)
}
