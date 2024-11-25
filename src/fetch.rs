use crate::error::FetchError;
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::{body::Buf, Request, Uri};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::from_reader;
use tokio::net::TcpStream;

#[derive(Debug, Deserialize)]
pub struct Data<T>(T);

#[allow(dead_code)]
pub async fn fetch_json<T: DeserializeOwned>(uri: String) -> Result<T, FetchError> {
    let uri = uri.parse::<Uri>().expect("Invalid URL");
    let host = uri.host().expect("Expected host to be a string");
    let port = uri.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::<TcpStream>::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = uri.authority().unwrap().clone();
    let req = Request::builder()
        .uri(uri)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;
    let res = sender.send_request(req).await?;

    let body = res.collect().await?.aggregate();
    let body_reader = body.reader();
    let response = from_reader(body_reader)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    //use crate::url::SanityURL;

    use super::*;
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Todo {
        userId: u32,
        id: u32,
        title: String,
        completed: bool,
    }

    #[tokio::test]
    async fn test_schema() {
        let json_data = r#"
    {
        "query": " *[_id == \"09139a58-311b-4779-8fa4-723f19242a8e\"]{\n   _id,\n    _createdAt\n }",
        "result": [
            {
                "_id": "09139a58-311b-4779-8fa4-723f19242a8e",
                "_createdAt": "2023-06-14T15:31:07Z"
            }
        ],
        "syncTags": [
            "s1:gj2hPQ"
        ],
        "ms": 4
    }
    "#;
        match serde_json::from_str::<QueryResult>(json_data) {
            Ok(_parsed) => {}
            Err(e) => eprintln!("Failed to parse JSON: {}", e),
        }
    }

    #[ignore]
    #[tokio::test]
    async fn fetch_get_test() {
        let uri = "http://jsonplaceholder.typicode.com/todos/1"
            .parse()
            .unwrap();
        let response: Result<Todo, FetchError> = fetch_json(uri).await;
        let response = match response {
            Ok(response) => Some(response),
            Err(_) => None,
        };
        assert_ne!(response, None);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[allow(non_snake_case)]
    pub struct Document {
        _id: String,
        _createdAt: String,
    }

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

    #[derive(Debug, Serialize, Deserialize)]
    struct FailedQuery {
        description: String,
        end: u64,
        query: String,
        start: u64,
        r#type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct FailedResult {
        error: FailedQuery,
    }

    #[ignore]
    #[tokio::test]
    async fn fetch_sanity_url() {
        //let query = r#"*[_type == "post"]{title, author}"#;
        //let sanity_url = SanityURL::new()
        //    .project_id(&"m9whymrq".to_string())
        //    .dataset(&"production".to_string())
        //    .api_version("v2023-05-01".to_string())
        //    .host("api.sanity.io".to_string())
        //    .query(&query.to_string())
        //    .build()
        //    .unwrap();
        let response: Result<FailedResult, FetchError> =
            fetch_json("https://m9whymrq.api.sanity.io/v2022-03-07/data/query/production?query=+*%5B_id+%3D%3D+%2209139a58-311b-4779-8fa4-723f19242a8e%22%5D%7B%0A+++_id%2C%0A++++_createdAt%0A+%7D".to_string()).await;
        let _response = match response {
            Ok(response) => Some(response),
            Err(e) => {
                println!("Error: {:?}", e.to_string());
                None
            }
        };
    }
}
