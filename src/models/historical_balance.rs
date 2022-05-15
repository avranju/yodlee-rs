use serde::{Deserialize, Serialize};

use super::Money;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBalance {
    pub date: Option<String>,
    pub is_asset: Option<bool>,
    pub balance: Option<Money>,
    pub as_of_date: Option<String>,
    pub data_source_type: Option<String>,
}
