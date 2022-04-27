use serde::{Deserialize, Serialize};

use super::{
    account_address::AccountAddress, email::Email, identifier::Identifier,
    phone_number::PhoneNumber,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountProfile {
    pub identifier: Option<Vec<Identifier>>,
    pub address: Option<Vec<AccountAddress>>,
    pub phone_number: Option<Vec<PhoneNumber>>,
    pub email: Option<Vec<Email>>,
}
