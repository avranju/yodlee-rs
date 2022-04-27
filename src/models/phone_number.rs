use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PhoneNumber {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub value: Option<String>,
}
