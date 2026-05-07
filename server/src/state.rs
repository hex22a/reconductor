use crate::features::user::register::RegisterFeature;

#[derive(Clone)]
pub struct AppState<R: RegisterFeature> {
    pub register_feature: R,
}
