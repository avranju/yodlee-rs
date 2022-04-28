use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{error::Error, models::User as UserModel, Client};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailsResponse {
    pub user: UserModel,
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

    pub async fn get_details(&mut self) -> Result<UserDetailsResponse, Error> {
        let access_token = self.client.ensure_token(&self.login_name).await?;
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
            Ok(res.json().await?)
        } else {
            Err(Error::Api(res.json().await?))
        }
    }

    pub async fn delete(&mut self) -> Result<(), Error> {
        let access_token = self.client.ensure_token(&self.login_name).await?;
        let (endpoint, api_version, http_client) = {
            let state = self.client.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "user/unregister"),
                state.api_version.clone(),
                state.http_client.clone(),
            )
        };

        let res = http_client
            .delete(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::Api(res.json().await?))
        }
    }
}
