use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full_name: Option<String>,
    pub first: Option<String>,
}
