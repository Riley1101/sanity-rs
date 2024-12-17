use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use sanity_rs::config::SanityConfig;
use sanity_rs::client::SanityClient;
use sanity_rs::create_client;
use sanity_rs::orm::ORM;
use sanity_rs::error::{ConfigurationError, RequestError};
use serde::{Deserialize, Serialize};
use futures::lock::Mutex;
use dotenv::dotenv;

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
#[get("/")]
async fn hello(client: web::Data<Mutex<SanityClient>>) -> impl Responder {
    let query = r#"
         *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
           _id,
           _createdAt
         }
        "#;
    let mut client = client.lock().await;
    let value: Result<QueryResult<Vec<Record>>, RequestError> = client.query(query).await.unwrap().json();
    println!("Value: {:?}", value);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
            <h1>Actix-web</h1>
            <p>Hello world!</p>
        "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let sanity_project_id = std::env::var("SANITY_PROJECT_ID")
        .map_err(|_| ConfigurationError::MissingProjectID)
        .expect("Missing project ID");
    let sanity_dataset = std::env::var("SANITY_DATASET")
        .map_err(|_| ConfigurationError::MissingDataset)
        .expect("Missing dataset");
    let config = SanityConfig::new(sanity_project_id, sanity_dataset);
    let client = create_client(config);
    let client_ref = web::Data::new(Mutex::new(client));
    HttpServer::new(move || {
        App::new()
            .app_data(client_ref.clone())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

