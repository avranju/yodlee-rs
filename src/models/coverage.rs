use serde::{Deserialize, Serialize};

use super::coverage_amount::CoverageAmount;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub amount: Option<Vec<CoverageAmount>>,
    pub plan_type: Option<String>,
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub start_date: Option<String>,
}
