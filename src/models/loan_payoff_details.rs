use serde::{Deserialize, Serialize};

use super::money::Money;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanPayoffDetails {
    pub pay_by_date: Option<String>,
    pub payoff_amount: Option<Money>,
    pub outstanding_balance: Option<Money>,
}
