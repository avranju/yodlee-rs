use serde::{Deserialize, Serialize};

use super::{
    account_address::AccountAddress, payment_bank_transfer_code::PaymentBankTransferCode,
    payment_identifier::PaymentIdentifier,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentProfile {
    pub identifier: Option<PaymentIdentifier>,
    pub address: Option<Vec<AccountAddress>>,
    pub payment_bank_transfer_code: Option<PaymentBankTransferCode>,
}
