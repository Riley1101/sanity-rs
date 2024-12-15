use leptos::prelude::*;
use sanity_rs::config::SanityConfig;
use sanity_rs::error::RequestError;
use sanity_rs::orm::ORM;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct QueryResult<T> {
    query: String,
    result: T,
    syncTags: Vec<String>,
    ms: u64,
}

impl<T> QueryResult<T> {
    fn new(query: String, result: T, sync_tags: Vec<String>, ms: u64) -> Self {
        Self {
            query,
            result,
            syncTags: sync_tags,
            ms,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Article {
    _id: String,
    _createdAt: String,
}

type ArticleList = QueryResult<Vec<Article>>;

async fn load_data() -> Vec<Article> {
    let sanity_project_id = "m9whymrq".to_string();
    let sanity_dataset = "production".to_string();
    let config = SanityConfig::new(sanity_project_id, sanity_dataset);
    let mut client = sanity_rs::create_client(config);

    let query = r#"
        *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
            _id,
            _createdAt
        }
    "#;

    let response = client.query(query).await.unwrap();
    let response = response.json::<ArticleList>();
    match response {
        Ok(result) => result.result,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let async_data = OnceResource::new(load_data());

    view! {
        <div>
            <h1> Hello, World! </h1>
        </div>
    }
}
