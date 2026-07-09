use std::sync::Arc;

use crate::features::{
    csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
    project::{create::CreateProjectFeature, get::GetProjectFeature, list::ListProjectsFeature},
    session::auth::AuthFeature,
    user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) register_feature: Arc<dyn RegisterFeature + Send + Sync>,
    pub(crate) login_feature: Arc<dyn LoginFeature + Send + Sync>,
    pub(crate) logout_feature: Arc<dyn LogoutFeature + Send + Sync>,
    pub(crate) csrf_feature: Arc<dyn TokenFeature + Send + Sync>,
    pub(crate) auth_feature: Arc<dyn AuthFeature + Send + Sync>,
    pub(crate) verify_csrf_feature: Arc<dyn VerifyCsrfFeature + Send + Sync>,
    pub(crate) create_project_feature: Arc<dyn CreateProjectFeature + Send + Sync>,
    pub(crate) get_project_feature: Arc<dyn GetProjectFeature + Send + Sync>,
    pub(crate) list_projects_feature: Arc<dyn ListProjectsFeature + Send + Sync>,
}
