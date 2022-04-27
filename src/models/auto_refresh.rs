use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoRefresh {
    pub additional_status: Option<String>,
    pub as_of_date: Option<String>,
    pub status: Option<String>,
}
