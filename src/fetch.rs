use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::{body::Buf, Request, Uri};
use hyper_util::rt::TokioIo;
use serde::de;
use serde_json::from_reader;
use tokio::net::TcpStream;

pub async fn fetch_json<T: de::DeserializeOwned>(
    uri: Uri,
) -> Result<T, Box<dyn std::error::Error>> {
    let host = uri.host().expect("Expected host to be a string");
    let port = uri.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await.unwrap();
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
        .body(Empty::<Bytes>::new())
        .unwrap();
    let res = sender.send_request(req).await?;
    let body = res.collect().await?.aggregate();

    let response = from_reader(body.reader())?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Todo {
        userId: u32,
        id: u32,
        title: String,
        completed: bool,
    }

    #[tokio::test]
    async fn fetch_test() {
        let uri = "http://jsonplaceholder.typicode.com/todos/1"
            .parse()
            .unwrap();
        let response: Result<Todo, Box<dyn std::error::Error>> = fetch_json(uri).await;
        let response = match response {
            Ok(response) => Some(response),
            Err(_) => None,
        };
        println!("{:?}", response);
        assert_ne!(response, None);
    }
}
