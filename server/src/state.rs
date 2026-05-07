use crate::infra::password::PasswordService;
use crate::persistence::db::user::UserRepository;

#[derive(Clone)]
pub struct AppState<P: PasswordService, U: UserRepository> {
    pub password_service: P,
    pub user_repository: U,
}
