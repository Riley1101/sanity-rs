# Rust client for Sanity.io

## Porting @sanity/client in Rust

This project is to easily query and parse documents from sanity.io and use it in your async rust runtime.

## Table of Contents

- [Features and Roadmap](#features-and-roadmap)
- [Getting started](#getting-started)
  - [Creating client](#creating-client)
  - [Querying documents](#querying-documents)
  - [Using ORM trait](#using-orm-trait)
- [PortableText to HTML](#portabletext-to-html)
- [Examples](#examples)

## Installation 

```
cargo add https://github.com/Riley1101/sanity-rs
```

## Features and Roadmap

- [🟢] Base
  - [x] Raw string query
  - [x] support String raw response
  - [x] serde integration with generics
- [🟢] PortableText Renderer
  - [x] Base sanity portable text
  - [🚧] Extract rust-portabletext crate
- [🚧] ORM
  - [x] get_by_id
  - [x] get_by_ids
  - [ ] more options
- [🔴] Mutations
- [🔴] Subscribe

## Getting started

### Creating client

```rust
use sanity_rs::client::{ SanityClient , create_client};
use sanity_rs::config::SanityConfig;

let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
    .map_err(|_| ConfigurationError::MissingProjectID)
    .expect("Missing project ID");
let sanity_dataset = std::env::var("SANITY_DATASET")
    .map_err(|_| ConfigurationError::MissingDataset)
    .expect("Missing dataset");
let config = SanityConfig::new(sanity_project_id, sanity_dataset);
let client = create_client(config);
```

### Querying documents

```rust
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
        *[_id == "0c80e597-8275-40b7-a3f5-1a3d3448bc39"][0]{
        _id,
        _createdAt
        }
    "#;
    let value: Result<QueryResult<Record>, RequestError> = client.query(query).await?.json();
    assert!(value.is_ok());
    Ok(())
}
```

### Using ORM trait

Currently there are a few ORM methods you can use to query documents.

```rust
#[tokio::test]
async fn orm_get_by_id() -> Result<(), RequestError> {
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
        .get_by_id("0c80e597-8275-40b7-a3f5-1a3d3448bc39")
        .body("{_id,_createdAt}")
        .send()
        .await?
        .json::<QueryResult<Record>>();
    assert!(v.is_ok());
    Ok(())
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
```

## PortableText to HTML

You can now use default `use sanity_rs::portabletext::renderer::ToHTML;` to render portable texts into HTML documents.

Each `PortableTextNode` can call `HTML` trait to call `html()` function.
Here is an example using default renderer.

```rust
let mut client = client.lock().await;
let v = client
    .get_by_id(&id)
    .body("{title,description,_id,body}")
    .send()
    .await
    .unwrap()
    .json::<QueryResult<ArticleWithBody>>();

let article = match v {
    Ok(res) => res.result,
    Err(_e) => ArticleWithBody {
        title: "Not Found".to_string(),
        description: "Article not found".to_string(),
        body: None,
        _id: "0".to_string(),
    },
};
let body = article.body.unwrap_or_default();
let body = ToHTML::new(body).render();
let response = format!("{result}", result = body); // result HTML string
```

## Examples

Checkout `examples/` folder for more examples.

- [Tokio example](examples/simple-tokio)
- [Full Server Side Rendering example with Actix Web ](examples/actix-web-static-page)
