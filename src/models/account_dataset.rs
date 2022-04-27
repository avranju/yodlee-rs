use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDataset {
    pub last_updated: Option<String>,
    pub update_eligibility: Option<String>,
    pub additional_status: Option<String>,
    pub next_update_scheduled: Option<String>,
    pub name: Option<String>,
    pub last_update_attempt: Option<String>,
}
