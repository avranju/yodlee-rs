// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use error::YodleeError;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Debug)]
pub struct State {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
    http_client: HttpClient,
    admin_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    state: Arc<RwLock<State>>,
}

impl Client {
    pub fn new(
        api_endpoint: String,
        api_version: String,
        admin_login_name: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Client {
            state: Arc::new(RwLock::new(State {
                api_endpoint,
                api_version,
                admin_login_name,
                client_id,
                client_secret,
                http_client: HttpClient::new(),
                admin_token: None,
            })),
        }
    }

    pub fn with_http_client(self, http_client: HttpClient) -> Self {
        self.state.write().unwrap().http_client = http_client;
        self
    }

    pub async fn open(&self) -> Result<(), YodleeError> {
        // retrieve the state we need to make the API call into new memory
        // so that we don't hold on to the mutex guard across await points
        let (ep, client_id, client_secret, http_client, api_version, admin_login_name) = {
            let state = self.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "auth/token"),
                state.client_id.clone(),
                state.client_secret.clone(),
                state.http_client.clone(),
                state.api_version.clone(),
                state.admin_login_name.clone(),
            )
        };

        // build a future that does the work necessary to always have a valid
        // admin token
        let this = self.clone();
        let token_future = async move {};

        let mut body = HashMap::new();
        body.insert("clientId", &client_id);
        body.insert("secret", &client_secret);

        let res = http_client
            .post(ep)
            .header("Api-Version", &api_version)
            .header("loginName", &admin_login_name)
            .form(&body)
            .send()
            .await?;

        if res.status().is_success() {
            let auth_response = res.json::<AuthResponse>().await?;

            Ok(())
        } else {
            return Err(YodleeError::Api);
        }
    }

    pub async fn close(self) -> Result<(), YodleeError> {
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    pub token: Token,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    pub access_token: String,
    pub issued_at: String,
    pub expires_in: i64,
}
