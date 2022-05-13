use serde::{Deserialize, Serialize};

use super::HistoricalBalance;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistory {
    historical_balances: Option<Vec<HistoricalBalance>>,
    id: Option<i64>,
}
