use crate::features::{
    csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
    session::auth::AuthFeature,
    user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub struct AppState<
    R: RegisterFeature,
    L: LoginFeature,
    O: LogoutFeature,
    T: TokenFeature,
    A: AuthFeature,
    C: VerifyCsrfFeature,
> {
    pub register_feature: R,
    pub login_feature: L,
    pub logout_feature: O,
    pub csrf_feature: T,
    pub auth_feature: A,
    pub verify_csrf_feature: C,
}
