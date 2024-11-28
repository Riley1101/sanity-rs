mod client;
mod config;
pub mod error;
mod url;
pub mod orm;

use serde::{Deserialize, Serialize};
use client::SanityClient;
use config::SanityConfig;
use dotenv::dotenv;
use error::ConfigurationError;

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


pub fn create_client() -> SanityClient {
    dotenv().ok();
    let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
        .map_err(|_| ConfigurationError::MissingProjectID)
        .expect("Missing project ID");
    let sanity_dataset = std::env::var("SANITY_DATASET")
        .map_err(|_| ConfigurationError::MissingDataset)
        .expect("Missing dataset");
    let config = SanityConfig::new(sanity_project_id, sanity_dataset);
    SanityClient::new(config)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    use error::RequestError;

    use super::*;

    #[allow(dead_code, non_snake_case)]
    #[derive(Deserialize, Debug, Serialize)]
    struct Document {
        _id: String,
        _createdAt: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct QueryResult {
        query: String,
        result: Vec<Document>,
        syncTags: Vec<String>,
        ms: usize,
    }

    #[tokio::test]
    async fn tokio_async_test() {
        let start = std::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(500)).await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(500));
    }

    #[tokio::test]
    async fn get_by_query() {
        let mut client = create_client();
        let query = r#"
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
        let value : Result<QueryResult, RequestError>= client.query(query).await.unwrap().json();
        assert_eq!(value.unwrap().result[0]._id, "09139a58-311b-4779-8fa4-723f19242a8e");
    }
}
