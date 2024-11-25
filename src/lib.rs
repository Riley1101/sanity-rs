mod client;
mod config;
mod error;
mod fetch;
mod query;
mod url;

use client::SanityClient;
use config::SanityConfig;
use dotenv::dotenv;
use error::ConfigurationError;

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
    use serde::Deserialize;
    use std::time::Duration;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct Document {
        _id: String,
    }

    #[tokio::test]
    async fn tokio_async_test() {
        let start = std::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(500)).await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(500));
    }

    #[tokio::test]
    #[ignore]
    async fn get_by_id() {
        let mut client = create_client();
        let body = "{ _id }";
        let value: Result<Document, error::FetchError> = client
            .get_by_id("09139a58-311b-4779-8fa4-723f19242a8e")
            .body(body)
            .send()
            .await;
        match value {
            Ok(value) => {
                println!("{:?}", value);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }
}
