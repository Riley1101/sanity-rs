use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::lock::Mutex;
use sanity_rs::client::SanityClient;
use sanity_rs::config::SanityConfig;
use sanity_rs::create_client;
use sanity_rs::error::{ConfigurationError, RequestError};
use sanity_rs::orm::ORM;
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
struct Slug {
    current: String,
    _type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    description: String,
    slug: Slug,
}

#[get("/")]
async fn home(client: web::Data<Mutex<SanityClient>>) -> impl Responder {
    let query = r###"
        *[_type=="article"][0..2]{
          title,
          description,
          slug
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
        response.push_str(&format!("<a href=/{}>Read More</a>", article.slug.current));
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

#[get("/{slug}")]
async fn article_route(req: HttpRequest, client: web::Data<Mutex<SanityClient>>) -> impl Responder {
    let slug: String = req.match_info().get("slug").unwrap().parse().unwrap();
    // let query = r###"
    //     *[_type=="article" && slug.current="some"][0]{
    //       title,
    //       slug
    //     }
    // "###;

    let mut client = client.lock().await;

    let string = format!("*[ _type=='article' &&'slug.current'=={}][0]{{
        title,
        description,
        slug
}}", slug);

    let result : Result<QueryResult<Article>, RequestError> = client.query(&string).await.unwrap().json();
    println!("{:?}", result);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"<h1>Article</h1>"#)
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
