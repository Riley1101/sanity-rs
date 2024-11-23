use crate::config::SanityConfig;
use reqwest::Client as ReqwestClient;
use std::fmt::Display;

#[derive(Default)]
pub struct RequestPayload {
    pub query: Option<String>,
    pub body: Option<String>,
}

impl RequestPayload {
    pub fn set_body(&mut self, body: &str) -> &Self {
        self.body = Some(body.to_string());
        self
    }
}

pub struct SanityClient {
    config: SanityConfig,
    client: ReqwestClient,
    payload: RequestPayload,
}

impl SanityClient {
    /// Create a new instance of the SanityClient
    pub fn new(config: SanityConfig) -> Self {
        Self {
            config,
            client: ReqwestClient::new(),
            payload: RequestPayload::default(),
        }
    }

    /// Generate the base URL for the Sanity API
    fn generate_base_url(&self) -> String {
        let api_host = self.config.api_host.as_deref().unwrap_or("api.sanity.io");
        let api_version = self.config.api_version.as_deref().unwrap_or("v2021-10-04");
        format!(
            "https://{}.{}/{}/data/query/{}?query=",
            self.config.project_id, api_host, api_version, self.config.dataset
        )
    }

    /// Get a single document from the Sanity API
    pub fn get_by_id(&mut self, id: &str) -> &mut Self {
        let url = self.generate_base_url();
        let query = format!("*[_id == \"{}\"]", id);
        match &self.payload.query {
            Some(_) => {}
            None => {
                self.payload.query = Some(format!("{}{}", url, query));
            }
        }
        self
    }

    /// Set the body of the request
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.payload.set_body(body);
        self
    }

    fn get_query(&self) -> String {
        let query = self.payload.query.as_deref().unwrap_or("");
        let body = self.payload.body.as_deref().unwrap_or("");
        format!("{}{}", query, body)
    }

    pub async fn send(&mut self) -> String {
        println!("Query: {}", self.get_query());
        let req = self.get_query();
        let client = reqwest::Client::new();
        let res = client.get(req).send().await.unwrap();
        res.text().await.unwrap()
    }
}

impl Display for SanityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SanityClient")
    }
}
