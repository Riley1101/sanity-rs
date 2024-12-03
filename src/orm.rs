use serde::de::DeserializeOwned;

use std::future::Future;
use crate::client::SanityClient;
use crate::error::RequestError;
use crate::url::SanityURL;

pub trait ORM {
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError>;
    fn get_by_id(&mut self, id: &str) -> &mut SanityClient;
    fn query(&mut self) -> impl Future<Output = ()>;
}

impl ORM for SanityClient {
    fn get_by_id(&mut self, id: &str) -> &mut SanityClient {
        let string = format!("*[_id == '{}']", id);
        let query = &mut self.payload.query;
        SanityURL::query(query, string.as_str());
        self
    }

    /// Parse the JSON response
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError> {
        let res = self.payload.query_result.as_ref().unwrap();
        let value: T = serde_json::from_str(res).map_err(RequestError::JsonParsingError)?;
        Ok(value)
    }

    async fn query(&mut self) {
        println!("Querying...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SanityConfig;
    use crate::create_client;
    use crate::error::ConfigurationError;
    use crate::error::RequestError;
    use crate::orm::ORM;
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};

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
    async fn get_by_id() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config = SanityConfig::new(sanity_project_id, sanity_dataset);

        let mut client = create_client(config);
        let _ = client.get_by_id("09139a58-311b-4779-8fa4-723f19242a8e").query().await;
        Ok(())
    }
}
