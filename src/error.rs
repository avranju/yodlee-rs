use thiserror::Error;

#[derive(Error, Debug)]
pub enum YodleeError {
    #[error("Yodlee API call failed.")]
    Api,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Unknown Yodlee API error.")]
    Unknown,
}
