use reqwest::header;

use crate::{error::Error, models::UserDetailsResponse, Client};

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
            let user_response: UserDetailsResponse = res.json().await?;
            Ok(user_response)
        } else {
            Err(Error::Api)
        }
    }
}
