use super::config::SanityConfig;
use super::{
    error::{RequestError, URLError},
    url::SanityURL,
};
use reqwest::Client as ReqwestClient;
use std::fmt::Display;
use url::Url;

/// Creates a new SanityClient.
///
/// # Example
///
///   ```
///   use sanity_rs::client::{ create_client, SanityClient };
///   use sanity_rs::config::SanityConfig;
///   use sanity_rs::error::ConfigurationError;
///
///   let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
///       .unwrap_or("project_id".to_string());
///   let sanity_dataset = std::env::var("SANITY_DATASET")
///       .unwrap_or("data_set".to_string());
///   let config = SanityConfig::new(sanity_project_id, sanity_dataset);
///   let mut client = create_client(config);
///   ```
///
/// # Arguments
///
/// * `config`: A `SanityConfig` struct containing the configuration for the client.
///
/// # Returns
///
/// A `SanityClient` instance.
///
/// # Panics
///
/// Panics if the client cannot be created.  The error from `SanityClient::new` will be included in the panic message.
pub fn create_client(config: SanityConfig) -> SanityClient {
    match SanityClient::new(config) {
        Ok(client) => client,
        Err(e) => panic!("Error creating client: {:?}", e),
    }
}


/// Represents the payload of a request.
///
/// This struct contains the URL, body, and query result of a request.
pub struct RequestPayload {
    /// The URL of the request.
    pub query: Url,
    /// The body of the request, if any.
    pub body: Option<String>,
    /// The result of the query, if any.  This is likely populated after processing the request.
    pub query_result: Option<String>,
}

impl Default for RequestPayload {
    fn default() -> Self {
        Self {
            query: Url::parse("https://api.sanity.io")
                .map_err(URLError::InvalidURL)
                .unwrap(),
            body: None,
            query_result: None,
        }
    }
}

impl RequestPayload {
    pub fn set_body(&mut self, body: &str) -> &Self {
        self.body = Some(body.to_string());
        self
    }
}

/// A client for interacting with the Sanity.io API.
///
/// This struct provides methods for making requests to the Sanity API.  It uses the `reqwest` crate for HTTP requests.
pub struct SanityClient {
    /// Configuration settings for the client.
    pub config: SanityConfig,
    /// The underlying HTTP client used for making requests.
    pub client: ReqwestClient,
    /// The payload to be sent with requests.  This is likely a struct containing data relevant to the request type.
    pub payload: RequestPayload,
}

impl SanityClient {
    /// Creates a new Sanity client.
    ///
    /// # Arguments
    ///
    /// * `config`: A `SanityConfig` struct containing the configuration for the client.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `SanityClient` or a `RequestError` if the URL parsing fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the URL cannot be parsed.
    pub fn new(config: SanityConfig) -> Result<Self, RequestError> {
        let url = SanityURL::new()
            .host(match &config.api_host {
                Some(host) => host.to_string(),
                None => "api.sanity.io".to_string(),
            })
            .use_cdn(config.use_cdn)
            .project_id(&config.project_id)
            .api_version(&config.api_version)
            .dataset(&config.dataset)
            .build()
            .map_err(RequestError::URLParsingError)?;
        let mut client = Self {
            config,
            client: ReqwestClient::new(),
            payload: RequestPayload::default(),
        };
        client.payload.query = url;
        Ok(client)
    }

    /// Sets the request body.
    ///
    /// This method allows you to set the request body for subsequent use.
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.payload.set_body(body);
        self
    }

    /// Send a query to the Sanity API
    pub async fn query(&mut self, body: &str) -> Result<&mut Self, RequestError> {
        let query = &mut self.payload.query;
        SanityURL::query(query, body);
        let v = self.client.get(query.as_str()).send();
        let v = v.await?.text().await?;
        self.payload.query_result = Some(v);
        Ok(self)
    }
}

impl Display for SanityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("SanityClient : {:?}", self.config.project_id))
    }
}
