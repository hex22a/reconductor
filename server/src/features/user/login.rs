use crate::{
    domain::error::ServerError,
    infra::password::PasswordService,
    persistence::{db::user::UserRepository, kv::session::SessionRepository},
};

pub trait LoginFeature {
    fn login(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<(), ServerError>> + Send;
}

#[derive(Clone)]
pub struct UserLoginFeature<P: PasswordService, R: UserRepository, S: SessionRepository> {
    password_service: P,
    user_repository: R,
    session_repository: S,
}

impl<P, R, S> LoginFeature for UserLoginFeature<P, R, S>
where
    P: PasswordService + Send + Sync,
    R: UserRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
{
    async fn login(&self, username: String, password: String) -> Result<(), ServerError> {
        todo!()
    }
}
