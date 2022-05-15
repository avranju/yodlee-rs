use serde::{Deserialize, Serialize};

use super::HistoricalBalance;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistory {
    pub historical_balances: Option<Vec<HistoricalBalance>>,
    pub id: Option<i64>,
}
