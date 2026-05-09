use crate::features::{csrf::token::TokenFeature, user::register::RegisterFeature};

#[derive(Clone)]
pub struct AppState<R: RegisterFeature, T: TokenFeature> {
    pub register_feature: R,
    pub csrf_feature: T,
}
