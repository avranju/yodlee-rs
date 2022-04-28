#![deny(rust_2018_idioms, warnings, missing_debug_implementations, unsafe_code)]

use std::sync::{Arc, RwLock};

use account::Account;
use error::Error;
use models::UserRegistration;
use reqwest::{header, Client as HttpClient};
use serde::{Deserialize, Serialize};
use token_manager::TokenManager;
use user::{User, UserDetailsResponse};

pub mod account;
pub mod error;
pub mod models;
mod token_manager;
pub mod user;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientState {
    Closed,
    Open,
}

#[derive(Debug)]
pub(crate) struct State {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
    http_client: HttpClient,
    state: ClientState,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) state: Arc<RwLock<State>>,
    pub(crate) token_manager: TokenManager,
}

impl Client {
    pub fn new(
        api_endpoint: String,
        api_version: String,
        admin_login_name: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        let state = Arc::new(RwLock::new(State {
            api_endpoint,
            api_version,
            admin_login_name,
            client_id,
            client_secret,
            http_client: HttpClient::new(),
            state: ClientState::Closed,
        }));
        let token_manager = TokenManager::new(state.clone());

        Client {
            state,
            token_manager,
        }
    }

    pub fn with_http_client(self, http_client: HttpClient) -> Self {
        self.state.write().unwrap().http_client = http_client;
        self
    }

    pub fn is_open(&self) -> bool {
        self.state.read().unwrap().state == ClientState::Open
    }

    pub async fn user(&self, login_name: String) -> Result<User, Error> {
        User::new(self.clone(), login_name).await
    }

    pub fn account(&self, login_name: String) -> Account {
        Account::new(self.clone(), login_name)
    }

    pub async fn open(&mut self) -> Result<(), Error> {
        // if we are already in open state, don't do nothing
        if self.is_open() {
            return Err(Error::AlreadyOpen);
        }

        self.token_manager
            .add_admin_login(self.state.read().unwrap().admin_login_name.clone())
            .await?;
        self.state.write().unwrap().state = ClientState::Open;
        Ok(())
    }

    pub async fn close(self) -> Result<(), Error> {
        self.token_manager.close().await;

        // NOTE: We don't need to do the following because self is dropped here.
        //self.state.write().unwrap().state = ClientState::Closed;

        Ok(())
    }

    pub async fn register_user(
        &mut self,
        user: UserRegistration,
    ) -> Result<UserDetailsResponse, Error> {
        // user registration must use the admin token to do its business
        let access_token = self.token_manager.get_admin_token().ok_or(Error::NoToken)?;

        let (endpoint, api_version, http_client) = {
            let state = self.state.read().unwrap();
            (
                // endpoint
                format!("{}/{}", state.api_endpoint, "user/register"),
                state.api_version.clone(),
                state.http_client.clone(),
            )
        };

        let req = UserRegistrationRequest { user };
        let res = http_client
            .post(endpoint)
            .header("Api-Version", api_version)
            .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
            .json(&req)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::Api(res.json().await?))
        }
    }

    pub(crate) async fn ensure_token(&mut self, login_name: &str) -> Result<String, Error> {
        match self.token_manager.get_token(login_name) {
            Some(token) => Ok(token),
            None => {
                self.token_manager.add_login(login_name.to_string()).await?;
                self.token_manager
                    .get_token(login_name)
                    .ok_or(Error::NoToken)
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct UserRegistrationRequest {
    user: UserRegistration,
}
