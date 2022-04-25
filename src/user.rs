use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{error::Error, Client};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestPreferences {
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
    pub preferences: Option<UserRequestPreferences>,
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
pub struct UserRegistrationResponse {
    pub preferences: Option<UserRequestPreferences>,
    pub session: Option<UserSession>,
    pub login_name: String,
    pub name: Option<UserName>,
    pub id: Option<i64>,
    pub role_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct User {
    client: Client,
}

impl User {
    pub fn new(client: Client) -> Self {
        User { client }
    }

    pub async fn register(
        &self,
        user: UserRegistration,
    ) -> Result<UserRegistrationResponse, Error> {
        if !self.client.is_open() {
            return Err(Error::Closed);
        }

        let (endpoint, api_version, http_client, admin_token) = {
            let state = self.client.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "user/register"),
                state.api_version.clone(),
                state.http_client.clone(),
                state.admin_token.clone().expect("Admin token is not set"),
            )
        };

        let res = http_client
            .post(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {admin_token}"))
            .json(&user)
            .send()
            .await?;

        if res.status().is_success() {
            let user_registration_response: UserRegistrationResponse = res.json().await?;
            Ok(user_registration_response)
        } else {
            Err(Error::Api)
        }
    }
}
