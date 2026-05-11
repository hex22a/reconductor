use crate::features::{
    csrf::token::TokenFeature,
    session::auth::AuthFeature,
    user::{login::LoginFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub struct AppState<R: RegisterFeature, L: LoginFeature, T: TokenFeature, A: AuthFeature> {
    pub register_feature: R,
    pub login_feature: L,
    pub csrf_feature: T,
    pub auth_feature: A,
}
