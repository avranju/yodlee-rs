use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::{Account as AccountModel, AccountHistory},
    Client,
};

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
        params: AccountParams<'_>,
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
                ("accountIds", params.account_ids.map(|e| e.join(","))),
                ("container", params.container.map(|s| s.to_string())),
                ("include", params.include.map(|s| s.to_string())),
                (
                    "providerAccountId",
                    params.provider_account_id.map(|s| s.to_string()),
                ),
                ("requestId", params.request_id.map(|s| s.to_string())),
                ("status", params.status.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::Api(res.json().await?))
        }
    }

    pub async fn get_historical_balances(
        &mut self,
        params: AccountHistoricalBalanceParams<'_>,
    ) -> Result<AccountHistoricalBalanceResponse, Error> {
        let access_token = self.client.ensure_token(&self.login_name).await?;

        let (endpoint, api_version, http_client) = {
            let state = self.client.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "accounts/historicalBalances"),
                state.api_version.clone(),
                state.http_client.clone(),
            )
        };

        let res = http_client
            .get(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .query(&[
                (
                    "includeCF",
                    params.include_carry_forward.map(|e| e.to_string()),
                ),
                ("fromDate", params.from_date.map(|s| s.to_string())),
                ("toDate", params.to_date.map(|s| s.to_string())),
                ("interval", params.interval.map(|s| s.to_string())),
                (
                    "accountReconType",
                    params.account_reconcile_type.map(|s| s.to_string()),
                ),
                ("skip", params.skip.map(|s| s.to_string())),
                ("top", params.top.map(|s| s.to_string())),
                ("accountId", params.account_id.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::Api(res.json().await?))
        }
    }
}

#[derive(Debug, Default)]
pub struct AccountParams<'a> {
    pub account_ids: Option<&'a [&'a str]>,
    pub container: Option<&'a str>,
    pub include: Option<&'a str>,
    pub provider_account_id: Option<&'a str>,
    pub request_id: Option<&'a str>,
    pub status: Option<&'a str>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountResponse {
    pub account: Option<Vec<AccountModel>>,
}

#[derive(Debug, Default)]
pub struct AccountHistoricalBalanceParams<'a> {
    pub include_carry_forward: Option<bool>,
    pub from_date: Option<&'a str>,
    pub to_date: Option<&'a str>,
    pub interval: Option<&'a str>,
    pub account_reconcile_type: Option<&'a str>,
    pub skip: Option<u32>,
    pub top: Option<u32>,
    pub account_id: Option<&'a str>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountHistoricalBalanceResponse {
    pub account: Option<Vec<AccountHistory>>,
}
