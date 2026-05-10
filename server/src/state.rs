use crate::features::{
    csrf::token::TokenFeature,
    user::{login::LoginFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub struct AppState<R: RegisterFeature, L: LoginFeature, T: TokenFeature> {
    pub register_feature: R,
    pub login_feature: L,
    pub csrf_feature: T,
}
