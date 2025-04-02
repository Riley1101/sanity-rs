use crate::config::SanityConfig;
use crate::{
    error::{RequestError, URLError},
    url::SanityURL,
};
use reqwest::Client as ReqwestClient;
use std::fmt::Display;
use url::Url;

/// Creates a new SanityClient.
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


pub struct RequestPayload {
    pub query: Url,
    pub body: Option<String>,
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

pub struct SanityClient {
    config: SanityConfig,
    pub client: ReqwestClient,
    pub payload: RequestPayload,
}

impl SanityClient {
    /// Create a new instance for the SanityClient
    /// Initialize a client instance based on Configuration
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

    /// Set the body of the request
    ///
    /// builder method for setting query body for later usecases.
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
