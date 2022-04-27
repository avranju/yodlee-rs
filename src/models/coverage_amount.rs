use serde::{Deserialize, Serialize};

use super::money::Money;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageAmount {
    pub cover: Option<Money>,
    pub unit_type: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub limit_type: Option<String>,
    pub met: Option<Money>,
}
