use crate::infra::password::Argon2Service;
use crate::persistence::db::user::PgUserRepository;

#[derive(Clone)]
pub struct AppState {
    pub password_service: Argon2Service,
    pub user_repository: PgUserRepository,
}
