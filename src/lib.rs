#![deny(rust_2018_idioms, warnings, missing_debug_implementations, unsafe_code)]

use std::sync::{Arc, RwLock};

use error::Error;
use reqwest::Client as HttpClient;
use token_manager::TokenManager;
use user::User;

pub mod error;
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
}
