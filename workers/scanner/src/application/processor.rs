use std::sync::Arc;

use uuid::Uuid;

use crate::{
    AppError,
    application::parser,
    domain::result::Host,
    features::{
        scan::{model::ScanStatus, update::UpdateScanFeature},
        scan_result::add::AddScanResultFeature,
    },
    infra::nmap::ScanRunner,
};

pub trait Processor {
    fn process(
        &self,
        scan_id: Uuid,
        target: &str,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct ScanProcessor<U: UpdateScanFeature, S: ScanRunner, A: AddScanResultFeature> {
    scan_runner: S,
    update_scan_feature: Arc<U>,
    add_scan_result_feature: Arc<A>,
}

impl<U, S, A> ScanProcessor<U, S, A>
where
    U: UpdateScanFeature,
    S: ScanRunner,
    A: AddScanResultFeature,
{
    pub fn new(
        scan_runner: S,
        update_scan_feature: Arc<U>,
        add_scan_result_feature: Arc<A>,
    ) -> Self {
        Self {
            scan_runner,
            update_scan_feature,
            add_scan_result_feature,
        }
    }
}

impl<U, S, A> Processor for ScanProcessor<U, S, A>
where
    U: UpdateScanFeature + Send + Sync,
    S: ScanRunner + Send + Sync,
    A: AddScanResultFeature + Send + Sync,
{
    async fn process(&self, scan_id: Uuid, target: &str) -> Result<(), AppError> {
        self.update_scan_feature
            .update_scan_status(scan_id, ScanStatus::InProgress)
            .await?;

        let xml = S::run(target).await?;
        let result = parser::parse(&xml)?;

        let hosts: Vec<Host> = result
            .hosts
            .into_iter()
            .filter(|h| h.status.state == "up")
            .collect();

        self.add_scan_result_feature
            .add_scan_results(scan_id, hosts)
            .await?;

        self.update_scan_feature
            .update_scan_status(scan_id, ScanStatus::Done)
            .await?;
        Ok(())
    }
}
