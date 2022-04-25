use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // TODO: Fetch the error code from the response body and store in this variant.
    #[error("Yodlee API call failed.")]
    Api,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Unknown Yodlee API error.")]
    Unknown,

    #[error("Client is already open.")]
    AlreadyOpen,

    #[error("Client is in closed state. No operations can be done.")]
    Closed,

    #[error("Could not cleanly close the client.")]
    Close,
}
