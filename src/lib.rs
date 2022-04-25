#![deny(rust_2018_idioms, warnings, missing_debug_implementations, unsafe_code)]

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use error::YodleeError;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc, time::sleep};

pub mod error;

#[derive(Debug)]
struct State {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
    http_client: HttpClient,
    admin_token: Option<String>,
    close_tx: Option<mpsc::Sender<()>>,
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
                close_tx: None,
            })),
        }
    }

    pub fn with_http_client(self, http_client: HttpClient) -> Self {
        self.state.write().unwrap().http_client = http_client;
        self
    }

    pub fn admin_token(&self) -> Option<String> {
        self.state.read().unwrap().admin_token.clone()
    }

    pub fn is_open(&self) -> bool {
        self.state.read().unwrap().close_tx.is_some()
    }

    pub async fn open(&self) -> Result<(), YodleeError> {
        // if we are already in open state, don't do nothing
        if self.is_open() {
            return Err(YodleeError::AlreadyOpen);
        }

        // retrieve the state we need to make the API call into new memory
        // so that we don't hold on to the mutex guard across await points
        let (endpoint, client_id, client_secret, http_client, api_version, admin_login_name, this) = {
            let state = self.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "auth/token"),
                state.client_id.clone(),
                state.client_secret.clone(),
                state.http_client.clone(),
                state.api_version.clone(),
                state.admin_login_name.clone(),
                self.clone(),
            )
        };

        // setup close channel
        let (close_tx, mut close_rx) = mpsc::channel(1);
        self.state.write().unwrap().close_tx = Some(close_tx);

        // get the admin token
        let mut token = get_admin_token(
            &http_client,
            &endpoint,
            &client_id,
            &client_secret,
            &api_version,
            &admin_login_name,
        )
        .await?;
        self.state.write().unwrap().admin_token = Some(token.access_token.clone());

        // build a future that does the work necessary to always have a valid
        // admin token
        let token_future = async move {
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_secs(token.expires_in)) => {
                        // refresh the admin token
                        match get_admin_token(
                            &http_client,
                            &endpoint,
                            &client_id,
                            &client_secret,
                            &api_version,
                            &admin_login_name,
                        ).await {
                            Ok(new_token) => {
                                token = new_token;
                                this.state.write().unwrap().admin_token = Some(token.access_token.clone());
                            },
                            Err(_) => {
                                let mut state = this.state.write().unwrap();
                                state.admin_token = None;
                                state.close_tx = None;
                                break;
                            }
                        }
                    }
                    _ = close_rx.recv() => {
                        this.state.write().unwrap().admin_token = None;
                        break;
                    }
                }
            }
        };

        tokio::spawn(token_future);

        Ok(())
    }

    pub async fn close(self) -> Result<(), YodleeError> {
        if let Some(close_tx) = self.state.write().unwrap().close_tx.take() {
            close_tx.send(()).await.map_err(|_| YodleeError::Close)?;
        }

        Ok(())
    }
}

async fn get_admin_token(
    http_client: &HttpClient,
    endpoint: &str,
    client_id: &str,
    client_secret: &str,
    api_version: &str,
    admin_login_name: &str,
) -> Result<Token, YodleeError> {
    let mut body = HashMap::new();
    body.insert("clientId", client_id);
    body.insert("secret", client_secret);

    let res = http_client
        .post(endpoint)
        .header("Api-Version", api_version)
        .header("loginName", admin_login_name)
        .form(&body)
        .send()
        .await?;

    if res.status().is_success() {
        let auth_response = res.json::<AuthResponse>().await?;

        Ok(auth_response.token)
    } else {
        Err(YodleeError::Api)
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
    pub expires_in: u64,
}
