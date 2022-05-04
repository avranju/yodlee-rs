use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use reqwest::{header, Client as HttpClient};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, Sender},
    time::sleep,
};

use crate::{error::Error, State};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AuthResponse {
    pub token: Token,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Token {
    pub access_token: String,
    pub issued_at: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
struct TokenEntry {
    token: Token,
    close_tx: Sender<()>,
}

#[derive(Debug, Clone)]
pub(crate) struct TokenManager {
    state: Arc<RwLock<State>>,
    admin_login_name: Option<String>,
    tokens: Arc<RwLock<HashMap<String, TokenEntry>>>,
}

impl TokenManager {
    pub(crate) fn new(state: Arc<RwLock<State>>) -> Self {
        TokenManager {
            state,
            admin_login_name: None,
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn add_admin_login(&mut self, login_name: String) -> Result<(), Error> {
        self.add_login(login_name.clone()).await?;
        self.admin_login_name = Some(login_name);
        Ok(())
    }

    pub(crate) fn get_token(&self, login_name: &str) -> Option<String> {
        self.tokens
            .read()
            .unwrap()
            .get(login_name)
            .map(|e| e.token.access_token.clone())
    }

    pub(crate) fn get_admin_token(&self) -> Option<String> {
        self.admin_login_name
            .as_ref()
            .and_then(|login_name| self.get_token(login_name.as_str()))
    }

    pub(crate) fn is_admin_login(&self, login_name: &str) -> bool {
        self.admin_login_name
            .as_ref()
            .map(|admin_login_name| login_name == admin_login_name.as_str())
            .unwrap_or(false)
    }

    pub(crate) fn is_user_login(&self, login_name: &str) -> bool {
        !self.is_admin_login(login_name)
    }

    pub(crate) async fn add_login(&mut self, login_name: String) -> Result<(), Error> {
        // if our token cache already has a login for this login name then
        // we don't need to fetch a fresh token
        if self.get_token(&login_name).is_some() {
            return Ok(());
        }

        // store the state we need to make the API call in new memory
        // so that we don't hold on to the mutex guard across await points
        let (endpoint, client_id, client_secret, http_client, api_version, this) = {
            let state = self.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "auth/token"),
                state.client_id.clone(),
                state.client_secret.clone(),
                state.http_client.clone(),
                state.api_version.clone(),
                self.clone(),
            )
        };

        // we need to use the admin access token when adding user logins
        let admin_access_token = if self.is_user_login(&login_name) {
            self.get_admin_token()
        } else {
            None
        };

        // get the admin access token
        let token = get_access_token(
            &http_client,
            &endpoint,
            &client_id,
            &client_secret,
            &api_version,
            &login_name,
            admin_access_token.as_deref(),
        )
        .await?;
        let mut expires_in = token.expires_in;

        // setup close channel
        let (close_tx, mut close_rx) = mpsc::channel(1);

        // save token entry
        let token_entry = TokenEntry {
            close_tx: close_tx.clone(),
            token,
        };
        self.tokens
            .write()
            .unwrap()
            .insert(login_name.clone(), token_entry);

        // build a future that does the work necessary to always have a valid
        // access token
        let token_future = async move {
            loop {
                tokio::select! {
                    // TODO: We should probably sleep for slightly less than the expiration
                    // duration instead of living on the edge.
                    _ = sleep(Duration::from_secs(expires_in)) => {
                        // refresh the access token
                        match get_access_token(
                            &http_client,
                            &endpoint,
                            &client_id,
                            &client_secret,
                            &api_version,
                            &login_name,
                            admin_access_token.as_deref(),
                        ).await {
                            Ok(new_token) => {
                                // save token expiry duration so we can schedule the next refresh
                                expires_in = new_token.expires_in;

                                // upsert the new token into our map
                                this.tokens.write().unwrap().entry(login_name.clone()).and_modify(|e| {
                                    e.token = new_token.clone();
                                }).or_insert_with(|| {
                                    TokenEntry {
                                        close_tx: close_tx.clone(),
                                        token: new_token,
                                    }
                                });
                            },
                            Err(_) => {
                                // TODO: This *could* have failed due to a transient error and we
                                // should really retry this before giving up. But let's cross that
                                // bridge when we come to it.
                                this.tokens.write().unwrap().remove(&login_name);
                                break;
                            }
                        }
                    }
                    _ = close_rx.recv() => {
                        this.tokens.write().unwrap().remove(&login_name);
                        break;
                    }
                }
            }
        };

        tokio::spawn(token_future);

        Ok(())
    }

    pub(crate) async fn close(self) {
        let channels = self
            .tokens
            .write()
            .unwrap()
            .iter()
            .map(|(_, e)| e.close_tx.clone())
            .collect::<Vec<_>>();
        for close_tx in channels {
            // we don't really care if this fails
            let _ = close_tx.send(()).await;
        }
    }
}

async fn get_access_token(
    http_client: &HttpClient,
    endpoint: &str,
    client_id: &str,
    client_secret: &str,
    api_version: &str,
    login_name: &str,
    admin_access_token: Option<&str>,
) -> Result<Token, Error> {
    let mut body = HashMap::new();
    body.insert("clientId", client_id);
    body.insert("secret", client_secret);

    let req = http_client
        .post(endpoint)
        .header("Api-Version", api_version)
        .header("loginName", login_name)
        .form(&body);

    let req = if let Some(admin_access_token) = admin_access_token {
        req.header(
            header::AUTHORIZATION,
            format!("Bearer {}", admin_access_token),
        )
    } else {
        req
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let auth_response = res.json::<AuthResponse>().await?;

        Ok(auth_response.token)
    } else {
        Err(Error::Api(res.json().await?))
    }
}
