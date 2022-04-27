use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{error::Error, models::Account as AccountModel, Client};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountResponse {
    account: Vec<AccountModel>,
}

#[derive(Debug)]
pub struct Account {
    login_name: String,
    client: Client,
}

impl Account {
    pub fn new(client: Client, login_name: String) -> Self {
        Account { login_name, client }
    }

    pub async fn get_accounts(
        &mut self,
        account_ids: Option<&[&str]>,
        container: Option<&str>,
        include: Option<&str>,
        provider_account_id: Option<&str>,
        request_id: Option<&str>,
        status: Option<&str>,
    ) -> Result<AccountResponse, Error> {
        let access_token = self.client.ensure_token(&self.login_name).await?;
        let (endpoint, api_version, http_client) = {
            let state = self.client.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "accounts"),
                state.api_version.clone(),
                state.http_client.clone(),
            )
        };

        let res = http_client
            .get(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .query(&[
                ("accountIds", account_ids.map(|e| e.join(","))),
                ("container", container.map(|s| s.to_string())),
                ("include", include.map(|s| s.to_string())),
                (
                    "providerAccountId",
                    provider_account_id.map(|s| s.to_string()),
                ),
                ("requestId", request_id.map(|s| s.to_string())),
                ("status", status.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::Api)
        }
    }
}
