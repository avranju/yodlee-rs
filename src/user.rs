use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{error::Error, Client};

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
pub struct UserDetailsResponse {
    pub user: UserResponse,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub preferences: Option<UserPreferences>,
    pub session: Option<UserSession>,
    pub login_name: String,
    pub name: Option<UserName>,
    pub id: Option<i64>,
    pub role_type: Option<String>,
    pub email: Option<String>,
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct User {
    login_name: String,
    client: Client,
}

impl User {
    pub async fn new(mut client: Client, login_name: String) -> Result<Self, Error> {
        client.token_manager.add_login(login_name.clone()).await?;
        Ok(User { login_name, client })
    }

    async fn ensure_token(&mut self) -> Result<String, Error> {
        match self.client.token_manager.get_token(&self.login_name) {
            Some(token) => Ok(token),
            None => {
                self.client
                    .token_manager
                    .add_login(self.login_name.clone())
                    .await?;
                self.client
                    .token_manager
                    .get_token(&self.login_name)
                    .ok_or(Error::NoToken)
            }
        }
    }

    pub async fn get_details(&mut self) -> Result<UserDetailsResponse, Error> {
        let access_token = self.ensure_token().await?;
        let (endpoint, api_version, http_client) = {
            let state = self.client.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "user"),
                state.api_version.clone(),
                state.http_client.clone(),
            )
        };

        let res = http_client
            .get(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await?;

        if res.status().is_success() {
            let user_response: UserDetailsResponse = res.json().await?;
            Ok(user_response)
        } else {
            Err(Error::Api)
        }
    }
}
