use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("URL Parsing error: {0}")]
    URLParsingError(#[from] URLError),

    #[error("Reqwest  error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParsingError(#[from] serde_json::Error),

    #[error("Missing Env error: {0}")]
    MissingEnvVarError(#[from] std::env::VarError),

    #[error("Request error: {0}")]
    StringParsingError(String),
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
