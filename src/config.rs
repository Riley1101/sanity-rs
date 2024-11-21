use std::fmt::Display;

enum ClientPerspective {
    Default,
    Draft,
    Live,
}

pub struct SanityConfig {
    project_id: String,
    dataset: String,
    use_cdn: bool,
    token: Option<String>,
    perspective: ClientPerspective,
    api_host: Option<String>,
    api_version: Option<String>,
    proxy: Option<String>,
    request_tag_prefix: Option<String>,
    ignore_browser_token_warning: bool,
    with_credentials: bool,
    timeout: Option<u64>,
    max_retries: Option<u64>,
    retry_delay: Option<fn(u64) -> u64>,
    use_project_hostname: bool,
}

impl SanityConfig {
    pub fn new(project_id: String, dataset: String) -> Self {
        Self {
            project_id,
            dataset,
            use_cdn: false,
            token: None,
            perspective: ClientPerspective::Default,
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
