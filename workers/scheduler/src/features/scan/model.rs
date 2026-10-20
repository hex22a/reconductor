use reconductor_core::{ScanId, ScanTarget};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DueScan {
    pub id: ScanId,
    pub target: ScanTarget,
    pub schedule: Option<String>,
}
