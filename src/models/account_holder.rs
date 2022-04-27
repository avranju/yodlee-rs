use serde::{Deserialize, Serialize};

use super::{identifier::Identifier, name::Name};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolder {
    pub identifier: Option<Vec<Identifier>>,
    pub gender: Option<String>,
    pub ownership: Option<String>,
    pub name: Option<Name>,
}
