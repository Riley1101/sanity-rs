use std::fmt::Display;

use crate::config::SanityConfig;

pub struct SanityClient {
    config: SanityConfig,
}

impl SanityClient {
    pub fn new(config: SanityConfig) -> Self {
        Self { config }
    }
}

impl Display for SanityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SanityClient")
    }
}
