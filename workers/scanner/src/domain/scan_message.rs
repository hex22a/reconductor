use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanMessage {
    pub id: Uuid,
    pub target: String,
}
