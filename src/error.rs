use thiserror::Error;

#[derive(Error, Debug)]
pub enum YodleeError {
    // TODO: Fetch the error code from the response body and store in this variant.
    #[error("Yodlee API call failed.")]
    Api,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Unknown Yodlee API error.")]
    Unknown,

    #[error("Client is already open.")]
    AlreadyOpen,

    #[error("Could not cleanly close the client.")]
    Close,
}
