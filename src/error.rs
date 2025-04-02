use thiserror::Error;

#[derive(Error, Debug)]
/// Represents errors that can occur during a request.
pub enum RequestError {
    /// Error during URL parsing.
    #[error("URL Parsing error: {0}")]
    URLParsingError(#[from] URLError),

    /// Error during the Reqwest request.
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error during JSON parsing.
    #[error("JSON parsing error: {0}")]
    JsonParsingError(#[from] serde_json::Error),

    /// Error when a required environment variable is missing.
    #[error("Missing Env error: {0}")]
    MissingEnvVarError(#[from] std::env::VarError),

    /// A generic string error for request failures.
    #[error("Request error: {0}")]
    StringParsingError(String),
}

#[derive(Error, Debug)]
/// Represents errors that can occur when working with URLs.
pub enum URLError {
    /// Indicates an invalid URL.  Wraps a `url::ParseError`.
    #[error("Invalid URL: {0}")]
    InvalidURL(#[from] url::ParseError),
}

#[derive(Error, Debug)]
/// Configuration errors that can occur during application initialization.
pub enum ConfigurationError {
    /// The project ID was not provided in the configuration.
    #[error("Missing project ID")]
    MissingProjectID,

    /// The dataset was not specified in the configuration.
    #[error("Missing dataset")]
    MissingDataset,
}
