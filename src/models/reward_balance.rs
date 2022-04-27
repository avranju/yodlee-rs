use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardBalance {
    pub expiry_date: Option<String>,
    pub balance_to_reward: Option<String>,
    pub balance_type: Option<String>,
    pub balance: Option<f64>,
    pub description: Option<String>,
    pub balance_to_level: Option<String>,
    pub units: Option<String>,
}
