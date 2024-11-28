use sanity_rs::config::SanityConfig;
use sanity_rs::create_client;
use sanity_rs::error::{ConfigurationError, RequestError};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct QueryResult<T> {
    query: String,
    result: T,
    syncTags: Vec<String>,
    ms: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    _id: String,
    _createdAt: String,
}

#[tokio::main]
async fn main() -> Result<(), RequestError> {
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
    let value: Result<QueryResult<Vec<Record>>, RequestError> = client.query(query).await?.json();

    if let Ok(result) = value {
        for record in result.result {
            println!("ID: {}, Created At: {}", record._id, record._createdAt);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn load_env_correctly() {
        dotenv().ok();
        let sanity_project_id = env::var("SANITY_PROJECT_ID");
        let sanity_dataset = env::var("SANITY_DATASET");
        assert!(sanity_project_id.is_ok());
        assert!(sanity_dataset.is_ok());
    }

    #[tokio::test]
    async fn fetch_a_document() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config: SanityConfig = SanityConfig::new(sanity_project_id, sanity_dataset);
        let mut client = create_client(config);
        let query = r#"
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"][0]{
           _id,
           _createdAt
         }
        "#;
        let value: Result<QueryResult<Record>, RequestError> = client.query(query).await?.json();
        assert!(value.is_ok());
        Ok(())
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Article {
        _id: String,
        title: String,
    }

    #[tokio::test]
    async fn get_articles() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config: SanityConfig = SanityConfig::new(sanity_project_id, sanity_dataset);
        let mut client = create_client(config);

        let get_first_article = r#"
        *[_type=="article"][0..2]{
            _id,
            title,
        }
        "#;
        let article: Result<QueryResult<Vec<Article>>, RequestError> =
            client.query(get_first_article).await?.json();
        assert!(article.is_ok());
        Ok(())
    }
}
