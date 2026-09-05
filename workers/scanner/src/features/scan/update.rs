use uuid::Uuid;

use crate::features::scan::{error::ScanError, model::ScanStatus, repository::ScanRepository};

pub trait UpdateScanFeature {
    fn update_scan_status(
        &self,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> impl Future<Output = Result<(), ScanError>> + Send;
}

pub struct UpdateScan<R: ScanRepository> {
    scan_repository: R,
}

impl<R: ScanRepository> UpdateScan<R> {
    pub fn new(scan_repository: R) -> Self {
        Self { scan_repository }
    }
}

impl<R: ScanRepository + Send + Sync> UpdateScanFeature for UpdateScan<R> {
    async fn update_scan_status(&self, scan_id: Uuid, status: ScanStatus) -> Result<(), ScanError> {
        self.scan_repository
            .update_scan_status(scan_id, status)
            .await
    }
}
