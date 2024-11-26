use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Request error: {0}")]
    URLParsingError(#[from] URLError),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Request error: {0}")]
    JsonParsingError(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    MissingEnvVarError(#[from] std::env::VarError),
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
