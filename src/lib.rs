mod client;
mod config;
mod error;
mod query;

use dotenv::dotenv;
use client::SanityClient;
use config::SanityConfig;

pub fn create_client() -> SanityClient {
    dotenv().ok();
    let sanity_project_id = std::env::var("SANITY_PROJECT_ID").expect("SANITY_PROJECT_ID must be set");
    let sanity_dataset = std::env::var("SANITY_DATASET").expect("SANITY_DATASET must be set");
    let config = SanityConfig::new(sanity_project_id, sanity_dataset);
    SanityClient::new(config)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[tokio::test]
    async fn paused_time() {
        let start = std::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(500)).await;
        println!("{:?}ms", start.elapsed().as_millis());
    }

    #[tokio::test]
    async fn it_works() {
        let mut client = create_client();
        let body = "";
        let value = &client
            .get_by_id("09139a58-311b-4779-8fa4-723f19242a8e")
            .body(body)
            .send().await;
        println!("Value: {}", value);
    }
}
