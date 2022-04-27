use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PaymentBankTransferCode {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
