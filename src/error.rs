use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // TODO: Fetch the error code from the response body and store in this variant.
    #[error(
        "Yodlee API call failed.\n\tCode: {}\n\tMessage: {}\n\tReference: {}",
        .0.error_code.as_deref().unwrap_or("Unknown"),
        .0.error_message.as_deref().unwrap_or("Unknown"),
        .0.reference_code.as_deref().unwrap_or("Unknown"),)]
    Api(ApiError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Unknown Yodlee API error.")]
    Unknown,

    #[error("Client is already open.")]
    AlreadyOpen,

    #[error("Client is in closed state. No operations can be done.")]
    Closed,

    #[error("No valid access token is available for the user.")]
    NoToken,

    #[error("Could not cleanly close the client.")]
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub reference_code: Option<String>,
}
