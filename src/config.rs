#[allow(dead_code)]
use std::fmt::Display;

pub enum ClientPerspective {
    Raw,
    Draft,
    Live,
}

pub struct SanityConfig {
    pub project_id: String,
    pub dataset: String,
    pub use_cdn: bool,
    pub token: Option<String>,
    pub perspective: ClientPerspective,
    pub api_host: Option<String>,
    pub api_version: Option<String>,
    pub proxy: Option<String>,
    pub request_tag_prefix: Option<String>,
    pub ignore_browser_token_warning: bool,
    pub with_credentials: bool,
    pub timeout: Option<u64>,
    pub max_retries: Option<u64>,
    pub retry_delay: Option<fn(u64) -> u64>,
    pub use_project_hostname: bool,
}

impl SanityConfig {
    pub fn new(project_id: String, dataset: String) -> Self {
        Self {
            project_id,
            dataset,
            use_cdn: false,
            token: None,
            perspective: ClientPerspective::Raw,
            api_host: None,
            api_version: None,
            proxy: None,
            request_tag_prefix: None,
            ignore_browser_token_warning: false,
            with_credentials: false,
            timeout: None,
            max_retries: None,
            retry_delay: None,
            use_project_hostname: false,
        }
    }
}

impl Display for SanityConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sanity Config Obj {}", self.project_id)
    }
}
