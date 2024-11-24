use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] hyper::Error),

    #[error("Invalid request error: {0}")]
    InvalidRequest(#[from] hyper::http::Error),

    #[error("Invalid response error: {0}")]
    InvalidResponse(#[from] serde_json::Error),
}
