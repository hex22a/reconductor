pub mod error;

use tokio::process::Command;

use crate::infra::nmap::error::NmapError;

pub trait ScanRunner {
    fn run(target: &str) -> impl Future<Output = Result<String, NmapError>> + Send;
}

pub struct NmapRunner;

impl ScanRunner for NmapRunner {
    async fn run(target: &str) -> Result<String, NmapError> {
        let output = Command::new("nmap")
            .args(["-sV", "-oX", "-", target])
            .output()
            .await?;

        if !output.status.success() {
            panic!("NMAP filed")
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}
