use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAddress {
    pub zip: Option<String>,
    pub country: Option<String>,
    pub address3: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub source_type: Option<String>,
    pub address1: Option<String>,
    pub street: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
