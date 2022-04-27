use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullAccountNumberList {
    pub payment_account_number: Option<String>,
    pub unmasked_account_number: Option<String>,
}
