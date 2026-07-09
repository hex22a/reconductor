use std::pin::Pin;

use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::features::scan::{dto::ScanDto, error::ScanError};

pub(crate) trait CreateScanFeature {
    fn create(
        &self,
        project_id: Uuid,
        target: IpNetwork,
        schedule: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + '_>>;
}
