use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::lock::Mutex;
use sanity_rs::client::create_client;
use sanity_rs::client::SanityClient;
use sanity_rs::config::SanityConfig;
use sanity_rs::error::{ConfigurationError, RequestError};
use sanity_rs::orm::ORM;
use sanity_rs::portabletext::blocks::{Node, Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct QueryResult<T> {
    query: String,
    result: T,
    #[serde(rename = "syncTags")]
    sync_tags: Vec<String>,
    ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    description: String,
    _id: String,
}

#[get("/")]
async fn home(client: web::Data<Mutex<SanityClient>>) -> impl Responder {
    let query = r###"
        *[_type=="post"][0..2]{
          _id,
          title,
          description,
        }
    "###;

    let mut client = client.lock().await;
    let result: Result<QueryResult<Vec<Article>>, RequestError> =
        client.query(query).await.unwrap().json();
    let articles = match result {
        Ok(res) => res.result,
        Err(_) => vec![],
    };
    let mut response = String::new();
    for article in articles {
        response.push_str(&format!("<h2>{}</h2>", article.title));
        response.push_str(&format!("<p>{}</p>", article.description));
        response.push_str(&format!("<a href=/{}>Read More</a>", article._id));
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct ArticleWithBody {
    title: String,
    description: String,
    _id: String,
    body: Option<Vec<Node>>,
}

#[get("/{id}")]
async fn article_route(req: HttpRequest, client: web::Data<Mutex<SanityClient>>) -> impl Responder {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();

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
        Err(_) => ArticleWithBody {
            title: "Not Found".to_string(),
            description: "Article not found".to_string(),
            body: None,
            _id: "0".to_string(),
        },
    };

    let body = match article.body {
        Some(body) => body,
        None => vec![],
    };

    let body = body
        .iter()
        .map(|node|  {
            println!("===================================");
            println!("{:?}", node);
            println!("===================================");
           return  node.html()
        })
        .collect::<Vec<String>>()
        .join("");


    let response = format!(
        "<h1>{title}</h1><p>{description}</p><hr>{result}",
        title = article.title,
        description = article.description,
        result = body
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
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
            .service(home)
            .service(article_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
