use serde::{Deserialize, Serialize};

use crate::config::SanityConfig;
use crate::url::SanityURL;
use reqwest::Client as ReqwestClient;
use std::fmt::Display;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct QueryResult {
    query: String,
    result: Vec<Record>,
    syncTags: Vec<String>,
    ms: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    _id: String,
    _createdAt: String,
}

#[derive(Default)]
#[allow(dead_code)]
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

#[allow(dead_code)]
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

    /// Set the body of the request
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.payload.set_body(body);
        self
    }

    pub async fn query(&mut self, body: &str) {
        let url = SanityURL::new()
            .project_id(&self.config.project_id)
            .dataset(&self.config.dataset)
            .query(body)
            .build();
        let value = self.client.get(url.unwrap().as_str()).send().await.unwrap();
        let res = value.text().await.unwrap();

        match serde_json::from_str::<QueryResult>(res.as_str()) {
            Ok(parsed) => {
                let documents = parsed.result;
                for doc in documents {
                    println!("_createdAt : {:?}", doc._createdAt);
                }
                println!("Parsed JSON successfully");
            }
            Err(e) => eprintln!("Failed to parse JSON: {}", e),
        }

        println!("{}", res);
    }
}

impl Display for SanityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SanityClient")
    }
}
