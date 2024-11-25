use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] hyper::Error),

    #[error("Invalid request error: {0}")]
    InvalidRequest(#[from] hyper::http::Error),

    #[error("Invalid response error: {0}")]
    InvalidResponse(#[from] serde_json::Error),

    #[error("Invalid host : {0}")]
    InvalidHost(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum URLError {
    #[error("Invalid URL : {0}")]
    InvalidURL(#[from] url::ParseError),
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Missing project ID")]
    MissingProjectID,

    #[error("Missing dataset")]
    MissingDataset,
}
