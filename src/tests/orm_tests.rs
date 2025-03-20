#[cfg(test)]
mod tests {
    use crate::config::SanityConfig;
    use crate::client::create_client;
    use crate::error::ConfigurationError;
    use crate::error::RequestError;
    use crate::orm::ORM;
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Record {
        _id: String,
        #[serde(rename = "_createdAt")]
        _created_at: String,
    }

    #[derive(Deserialize, Debug, Serialize)]
    struct Document {
        _id: String,
        #[serde(rename = "_createdAt")]
        _created_at: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct QueryResult {
        query: String,
        result: Vec<Document>,
        #[serde(rename = "syncTags")]
        sync_tags: Vec<String>,
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
        let v = client
            .get_by_id("09139a58-311b-4779-8fa4-723f19242a8e")
            .body("{_id,_createdAt}")
            .send()
            .await;
        assert!(v.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn get_by_ids() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config = SanityConfig::new(sanity_project_id, sanity_dataset);

        let mut client = create_client(config);
        let ids = vec![
            "09139a58-311b-4779-8fa4-723f19242a8e",
            "ad79d8a3-35a9-4ac6-ab5b-cc0c62288b37",
        ];
        let v = client
            .get_by_ids(&ids)
            .body("")
            .send()
            .await?;
        let v = v.json::<QueryResult>();
        assert!(v.is_ok());
        Ok(())
    }
}
