pub mod parser;

use tokio::process::Command;

pub async fn run(target: &str) -> anyhow::Result<String> {
    let output = Command::new("nmap")
        .args(["-sV", "-oX", "-", target])
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!(
            "nmap failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8(output.stdout)?)
}
