use sanity_rs::create_client;
use sanity_rs::error::FetchError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Document {
    _id: String,
}

#[tokio::main]
async fn main() {
    // Open a connection to the mini-redis address.
    let mut client = create_client();
    let document: Result<Document, FetchError> = client
        .get_by_id("09139a58-311b-4779-8fa4-723f19242a8e")
        .body("{ _id }")
        .send()
        .await
        .unwrap();

    println!("got value from the server; result={:?}", "Hello world");
}
