#![allow(dead_code)]
use std::fmt::Display;

use crate::error::URLError;
use url::Url;

pub struct Query {
    condition: String,
    body: String,
}

#[derive(Debug)]
pub struct SanityURL {
    project_id: String,
    host: String,
    api_version: String,
    dataset: String,
    query: String,
}

impl Display for SanityURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sanity URL: https://{}.{}", self.project_id, self.host)
    }
}

impl SanityURL {
    pub fn new() -> Self {
        Self {
            project_id: "".to_string(),
            host: "api.sanity.io".to_string(),
            api_version: "v2022-03-07".to_string(),
            dataset: "production".to_string(),
            query: "".to_string(),
        }
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host.to_string();
        self
    }

    pub fn api_version(&mut self, api_version: &String) -> &mut Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_base() {
        let sanity_url = SanityURL::new()
            .project_id(&"abc123".to_string())
            .dataset(&"production".to_string())
            .api_version(&"v2022-03-07".to_string())
            .host("api.sanity.io".to_string())
            .build()
            .unwrap();

        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2022-03-07/data/query/production"
        );
    }

    #[test]
    fn query_test_one() -> Result<(), URLError> {
        let query = r#"
        *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
            _id,
            _type,
            _createdAt,
            _updatedAt
        }"#;
        let mut sanity_url = SanityURL::new()
            .project_id(&"abc123".to_string())
            .dataset(&"production".to_string())
            .api_version(&"v2022-03-07".to_string())
            .host("api.sanity.io".to_string())
            .build()?;
        SanityURL::query(&mut sanity_url, query);
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2022-03-07/data/query/production?query=*[_id%20==%20%2209139a58-311b-4779-8fa4-723f19242a8e%22]{_id,_type,_createdAt,_updatedAt}"
        );
        Ok(())
    }

    #[test]
    fn query_with_filter() -> Result<(), URLError> {
        let query = r#"
        *[type == "post" && published == true]{
            title,
            author,
            categories[]->title
        }"#;
        let mut sanity_url = SanityURL::new()
            .project_id(&"abc123".to_string())
            .dataset(&"blog".to_string())
            .api_version(&"v2023-01-01".to_string())
            .host("api.sanity.io".to_string())
            .build()?;
        SanityURL::query(&mut sanity_url, query);
        assert_eq!(
            sanity_url.as_str(),
                "https://abc123.api.sanity.io/v2023-01-01/data/query/blog?query=*[type%20==%20%22post%22%20&&%20published%20==%20true]{title,author,categories[]-%3Etitle}"
        );
        Ok(())
    }

    #[test]
    fn empty_query() -> Result<(), URLError> {
        let mut sanity_url = SanityURL::new()
            .project_id(&"xyz456".to_string())
            .dataset(&"test".to_string())
            .api_version(&"v2023-05-01".to_string())
            .host("api.sanity.io".to_string())
            .build()?;
        SanityURL::query(&mut sanity_url, "");
        assert_eq!(
            sanity_url.as_str(),
            "https://xyz456.api.sanity.io/v2023-05-01/data/query/test"
        );
        Ok(())
    }

    #[test]
    fn query_with_special_characters() -> Result<(), URLError> {
        let query = r#"
        *[name == "O'Reilly" && price < 100.0]{
            name,
            price
        }"#;
        let mut sanity_url = SanityURL::new()
            .project_id(&"abc123".to_string())
            .dataset(&"store".to_string())
            .api_version(&"v2023-05-01".to_string())
            .host("api.sanity.io".to_string())
            .build()?;
        SanityURL::query(&mut sanity_url, query);
        assert_eq!(
            sanity_url.as_str(),
             "https://abc123.api.sanity.io/v2023-05-01/data/query/store?query=*[name%20==%20%22O%27Reilly%22%20&&%20price%20%3C%20100.0]{name,price}",
        );
        Ok(())
    }

    #[test]
    fn one_line_query() -> Result<(), URLError> {
        let query = r#"*[_type == "post"]{title, author}"#;
        let mut sanity_url = SanityURL::new()
            .project_id(&"abc123".to_string())
            .dataset(&"blog".to_string())
            .api_version(&"v2023-05-01".to_string())
            .host("api.sanity.io".to_string())
            .build()?;
        SanityURL::query(&mut sanity_url, query);
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2023-05-01/data/query/blog?query=*[_type%20==%20%22post%22]{title,author}"
        );
        Ok(())
    }
}
