use crate::features::{
    csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
    session::auth::AuthFeature,
    user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub(crate) struct AppState<
    R: RegisterFeature,
    L: LoginFeature,
    O: LogoutFeature,
    T: TokenFeature,
    A: AuthFeature,
    C: VerifyCsrfFeature,
> {
    pub(crate) register_feature: R,
    pub(crate) login_feature: L,
    pub(crate) logout_feature: O,
    pub(crate) csrf_feature: T,
    pub(crate) auth_feature: A,
    pub(crate) verify_csrf_feature: C,
}
