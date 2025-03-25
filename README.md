# Rust client for Sanity.io

## Porting @sanity/client in Rust

This project is to easily query and parse documents from sanity.io and use it in your async rust runtime.

## Features and todo

- [🟢] Base
  - [x] Raw string query
  - [x] support String raw response
  - [x] serde integration with generics
- [🚧] ORM
  - [x] ORM trait
- [🚧] Portable Text toHTML
  - [x] Base sanity portable text
- [🔴] Actions
- [🔴] Subscribe

## Example

```rust
use sanity_rs::client::create_client;
use sanity_rs::error::RequestError;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<(), RequestError> {
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
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
    let value: Result<QueryResult, RequestError> = client.query(query).await?.json();

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
    #[tokio::test]
    async fn fetch_a_document() {
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
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
        let value: Result<QueryResult, RequestError> = client.query(query).await.unwrap().json();
        assert!(value.is_ok());
    }

    #[tokio::test]
    async fn orm_get_by_ids() -> Result<(), RequestError> {
        dotenv().ok();
        let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
            .map_err(|_| ConfigurationError::MissingProjectID)
            .expect("Missing project ID");
        let sanity_dataset = std::env::var("SANITY_DATASET")
            .map_err(|_| ConfigurationError::MissingDataset)
            .expect("Missing dataset");
        let config: SanityConfig = SanityConfig::new(sanity_project_id, sanity_dataset);
        let mut client = create_client(config);

        let v = client
            .get_by_ids(&[
                "09139a58-311b-4779-8fa4-723f19242a8e",
                "09139a58-311b-4779-8fa4-723f19242a8e",
            ])
            .body("{_id,_createdAt}")
            .send()
            .await?
            .json::<QueryResult<Vec<Record>>>();
        assert!(v.is_ok());
        Ok(())
    }
}
```

## Found bugs

[ ] Multiple query condition tends to a bit messy when you have like "slug.current"
  - checkout query builder at the `./src/url.rs` for the `SanityURL` struct
  - Just use orm with get_by_id for now you know it works better :3
