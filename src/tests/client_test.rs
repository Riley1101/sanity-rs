#[cfg(test)]
pub mod tests {
    use dotenv::dotenv;
    use crate::client::create_client;
    use crate::config::SanityConfig;
    use crate::error::{ConfigurationError, RequestError};
    use crate::orm::ORM;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Debug, Serialize, Deserialize)]
    struct Record {
        #[serde(rename = "_id")] 
        _id: String,
        #[serde(rename = "_createdAt")] 
        created_at: String,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct Document {
        #[serde(rename = "_id")] 
        id: String,
        #[serde(rename = "_createdAt")] 
        _created_at: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct QueryResult {
        query: String,
        result: Vec<Document>,
        #[serde(rename = "syncTags")]
        sync_tags: Vec<String>,
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
            value?.result[0].id,
            "09139a58-311b-4779-8fa4-723f19242a8e"
        );
        Ok(())
    }
}
