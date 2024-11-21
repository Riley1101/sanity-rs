mod client;
mod config;
mod error;
mod query;

use client::SanityClient;
use config::SanityConfig;

pub fn create_client() -> SanityClient {
    let config = SanityConfig::new("m9whymrq".to_string(), "dataset".to_string());
    SanityClient::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut client = create_client();
        let body = "";
        let document = &client
            .get_by_id("09139a58-311b-4779-8fa4-723f19242a8e")
            .body(body)
            .send();
    }
}
