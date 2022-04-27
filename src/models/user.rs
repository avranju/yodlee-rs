use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub date_format: Option<String>,
    pub time_zone: Option<String>,
    pub currency: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAddress {
    pub zip: Option<String>,
    pub country: Option<String>,
    pub address3: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub address1: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserName {
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full_name: Option<String>,
    pub first: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRegistration {
    pub preferences: Option<UserPreferences>,
    pub address: Option<UserAddress>,
    pub login_name: String,
    pub name: Option<UserName>,
    pub email: Option<String>,
    pub segment_name: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSession {
    pub user_session: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub preferences: Option<UserPreferences>,
    pub session: Option<UserSession>,
    pub login_name: String,
    pub name: Option<UserName>,
    pub id: Option<i64>,
    pub role_type: Option<String>,
    pub email: Option<String>,
    pub segment_name: Option<String>,
}
