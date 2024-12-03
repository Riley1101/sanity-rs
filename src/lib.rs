mod client;
pub mod config;
pub mod error;
pub mod orm;
mod url;

use client::SanityClient;
use config::SanityConfig;

pub fn create_client(config: SanityConfig) -> SanityClient {
    match SanityClient::new(config) {
        Ok(client) => client,
        Err(e) => panic!("Error creating client: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use error::ConfigurationError;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    use crate::error::RequestError;
    use orm::ORM;

    use super::*;

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct Record {
        _id: String,
        _createdAt: String,
    }

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
    async fn get_by_query() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config = SanityConfig::new(sanity_project_id, sanity_dataset);

        let mut client = create_client(config);
        let query = r#"
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
        let value = client.query(query).await?.json::<QueryResult>();
        assert_eq!(
            value?.result[0]._id,
            "09139a58-311b-4779-8fa4-723f19242a8e"
        );
        Ok(())
    }
}
