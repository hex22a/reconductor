use std::sync::Arc;

use crate::features::{
    csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
    project::{create::CreateProjectFeature, get::GetProjectFeature, list::ListProjectsFeature},
    scan::create::CreateScanFeature,
    session::auth::AuthFeature,
    user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
};

#[derive(Clone)]
pub struct AppState {
    pub register_feature: Arc<dyn RegisterFeature + Send + Sync>,
    pub login_feature: Arc<dyn LoginFeature + Send + Sync>,
    pub logout_feature: Arc<dyn LogoutFeature + Send + Sync>,
    pub csrf_feature: Arc<dyn TokenFeature + Send + Sync>,
    pub auth_feature: Arc<dyn AuthFeature + Send + Sync>,
    pub verify_csrf_feature: Arc<dyn VerifyCsrfFeature + Send + Sync>,
    pub create_project_feature: Arc<dyn CreateProjectFeature + Send + Sync>,
    pub get_project_feature: Arc<dyn GetProjectFeature + Send + Sync>,
    pub list_projects_feature: Arc<dyn ListProjectsFeature + Send + Sync>,
    pub create_scan_feature: Arc<dyn CreateScanFeature + Send + Sync>,
}
