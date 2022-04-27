use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BankTransferCode {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
