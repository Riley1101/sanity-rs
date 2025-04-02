use std::fmt::Display;

/// Represents the perspective of a client.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientPerspective {
    /// Raw data, unprocessed.
    Raw,
    /// Draft version, not yet finalized.
    Draft,
    /// Live version, currently in use.
    Live,
}

/// Configuration struct for the Sanity client.
///
/// This struct holds all the necessary configuration options for interacting with the Sanity API.
pub struct SanityConfig {
    /// Your Sanity project ID.  This is required.
    pub project_id: String,
    /// Your Sanity dataset. This is required.
    pub dataset: String,
    /// Whether to use the Sanity CDN. Defaults to `false`.
    pub use_cdn: bool,
    /// Your Sanity API token.  This is optional, but recommended for authenticated requests.
    pub token: Option<String>,
    /// The perspective to use when querying Sanity.  This determines which parts of your schema are visible.  See the Sanity documentation for more details.
    pub perspective: ClientPerspective,
    /// The host to use for the Sanity API.  Defaults to the standard Sanity API host.
    pub api_host: Option<String>,
    /// The API version to use. Defaults to the latest version.
    pub api_version: Option<String>,
    /// Proxy settings to use for requests.  Specifies the proxy URL as a string.
    pub proxy: Option<String>,
    /// Prefix for request tags.  Useful for debugging and monitoring.
    pub request_tag_prefix: Option<String>,
    /// Whether to ignore warnings about the browser token. Defaults to `false`.
    pub ignore_browser_token_warning: bool,
    /// Whether to send cookies with requests. Defaults to `false`.  This is primarily relevant for authenticated requests where cookies might be used for session management.
    pub with_credentials: bool,
    /// Timeout for API requests in milliseconds.
    pub timeout: Option<u64>,
    /// Maximum number of retries for failed API requests.
    pub max_retries: Option<u64>,
    /// Function to calculate the delay between retries.  The input is the retry number (starting at 0), and the output is the delay in milliseconds.
    pub retry_delay: Option<fn(u64) -> u64>,
    /// Whether to use the project hostname in the API URL. Defaults to `false`.
    pub use_project_hostname: bool,
}

impl SanityConfig {
    pub fn new(project_id: String, dataset: String) -> Self {
        Self {
            project_id,
            dataset,
            use_cdn: true,
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
