use thiserror::Error;
#[derive(Error, Debug)]
pub enum APIError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned error: {0}")]
    Api(String),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

