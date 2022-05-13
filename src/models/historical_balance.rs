use serde::{Deserialize, Serialize};

use super::Money;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBalance {
    date: Option<String>,
    is_asset: Option<bool>,
    balance: Option<Money>,
    as_of_date: Option<String>,
    data_source_type: Option<String>,
}
