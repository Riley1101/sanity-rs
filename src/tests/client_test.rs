#[cfg(test)]
pub mod tests {
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
}
