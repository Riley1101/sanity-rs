use crate::config::SanityConfig;
use crate::error::RequestError;
use crate::url::SanityURL;

use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use std::fmt::Display;

/// Request Payload for temporary storing query and request body
#[derive(Default)]
#[allow(dead_code)]
pub struct RequestPayload {
    pub query: Option<String>,
    pub body: Option<String>,
    pub result: Option<String>,
}

impl RequestPayload {
    pub fn set_body(&mut self, body: &str) -> &Self {
        self.body = Some(body.to_string());
        self
    }
}

#[allow(dead_code)]
pub struct SanityClient {
    config: SanityConfig,
    client: ReqwestClient,
    payload: RequestPayload,
}

impl SanityClient {
    /// Create a new instance for the SanityClient
    ///
    /// Initialize a client instance based on Configuration
    ///
    /// ## Example
    ///
    /// ```
    /// let sanity_project_id = abc123
    /// let sanity_dataset = development
    /// let config = SanityConfig::new(sanity_project_id, sanity_dataset);
    /// ```
    pub fn new(config: SanityConfig) -> Self {
        Self {
            config,
            client: ReqwestClient::new(),
            payload: RequestPayload::default(),
        }
    }

    /// Set the body of the request
    /// 
    /// builder method for setting query body for later usecases.
    ///
    /// ## Example
    ///
    /// ```
    /// let body = r#"
    ///  {
    ///    _id,
    ///    _createdAt,
    ///    _title,
    ///  }
    /// "#;
    ///
    /// client.get_by_id().body(body).send().await;
    /// ```
    fn body(&mut self, body: &str) -> &mut Self {
        self.payload.set_body(body);
        self
    }

    /// Send a query to the Sanity API
    pub async fn query(&mut self, body: &str) -> Result<&mut Self, RequestError> {
        let url = SanityURL::new()
            .project_id(&self.config.project_id)
            .dataset(&self.config.dataset)
            .query(body)
            .build()
            .map_err(RequestError::URLParsingError)?;

        let value = self.client.get(url.as_str()).send().await?;
        let res = value.text().await?;
        self.payload.result = Some(res);
        Ok(self)
    }

    /// Get the response as a string
    pub fn string(&self) -> Result<String, RequestError> {
        self.payload
            .result
            .as_ref()
            .map_or(
                Err(RequestError::StringParsingError("No response found".to_string())),
                |res| Ok(res.to_string()),
            )
    }

    /// Parse the JSON response
    pub fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError> {
        let res = self.payload.result.as_ref().unwrap();
        let value: T = serde_json::from_str(res).map_err(RequestError::JsonParsingError)?;
        Ok(value)
    }
}

impl Display for SanityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SanityClient")
    }
}
