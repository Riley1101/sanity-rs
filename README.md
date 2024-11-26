# Rust client for Sanity.io

## Porting @sanity/client in Rust

This project is to easily query and parse documents from sanity.io and use it in your async rust runtime.

## Features and todo

- [🟢] Query
- [🚧] ORM
- [🔴] Actions
- [🔴] Subscribe

## Example

```rust
use sanity_rs::create_client;
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
    let mut client = create_client();
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
        let mut client = create_client();
        let query = r#"
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
        let value: Result<QueryResult, RequestError> = client.query(query).await.unwrap().json();
        assert!(value.is_ok());
    }
}

```
