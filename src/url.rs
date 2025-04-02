use super::error::URLError;
use std::fmt::Display;
use url::Url;

#[derive(Debug)]
pub struct SanityURL {
    project_id: String,
    host: String,
    api_version: String,
    dataset: String,
}

impl SanityURL {
    pub fn new() -> Self {
        Self {
            project_id: "".to_string(),
            host: "api.sanity.io".to_string(),
            api_version: "v2022-03-07".to_string(),
            dataset: "production".to_string(),
        }
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host.to_string();
        self
    }

    pub fn api_version(&mut self, api_version: &Option<String>) -> &mut Self {
        let api_version = match api_version {
            Some(version) => version,
            None => &"v2022-03-07".to_string(),
        };
        self.api_version = api_version.to_string();
        self
    }

    pub fn project_id(&mut self, project_id: &String) -> &mut Self {
        self.project_id = project_id.to_string();
        self
    }

    pub fn use_cdn(&mut self, use_cdn: bool) -> &mut Self {
        self.host = if use_cdn {
            "apicdn.sanity.io".to_string()
        } else {
            "api.sanity.io".to_string()
        };
        self
    }

    pub fn dataset(&mut self, dataset: &String) -> &mut Self {
        self.dataset = dataset.to_string();
        self
    }

    pub fn build(&mut self) -> Result<Url, URLError> {
        let url = Url::parse(&format!(
            "https://{}.{}/{}/data/query/{}",
            self.project_id, self.host, self.api_version, self.dataset,
        ))
        .map_err(URLError::InvalidURL)?;
        Ok(url)
    }

    pub fn query(url: &mut Url, query: &str) {
        if query.is_empty() || query.len() < 3 {
            url.set_query(None);
            return;
        }

        let cond_start = query.find("*[").unwrap_or(0) + 2;
        let cond_end = query.find("]").unwrap_or(0);
        let condition = query[cond_start..cond_end].to_string();
        let mut body = query[cond_end + 1..]
            .to_string()
            .split_whitespace()
            .collect::<String>();
        if condition.is_empty() {
            url.set_query(None);
            return;
        }
        if body.is_empty() {
            body = "".to_string();
        }
        let query = format!("*[{}]{}", condition, body);
        url.set_query(Some(&format!("query={}", query)));
    }
}

impl Display for SanityURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sanity URL: https://{}.{}", self.project_id, self.host)
    }
}
