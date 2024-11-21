mod client;
mod config;
mod error;

use client::SanityClient;
use config::SanityConfig;

pub fn create_client() {
    let config = SanityConfig::new("project_id".to_string(), "dataset".to_string());
    let client = SanityClient::new(config);

    println!("{}", client);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        create_client();
    }
}
