use crate::error::URLError;
use url::Url;

pub struct SanityURL {
    project_id: String,
    host: String,
    api_version: String,
    dataset: String,
    query: String,
}

impl SanityURL {
    pub fn new() -> Self {
        Self {
            project_id: "".to_string(),
            host: "api.sanity.io".to_string(),
            api_version: "v2021-10-04".to_string(),
            dataset: "production".to_string(),
            query: "".to_string(),
        }
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host.to_string();
        self
    }

    pub fn api_version(&mut self, api_version: String) -> &mut Self {
        self.api_version = api_version.to_string();
        self
    }

    pub fn project_id(&mut self, project_id: String) -> &mut Self {
        self.project_id = project_id.to_string();
        self
    }

    pub fn dataset(&mut self, dataset: String) -> &mut Self {
        self.dataset = dataset.to_string();
        self
    }

    pub fn query(&mut self, query: String) -> &mut Self {
        self.query = query.replace(" ", "").trim().replace("\n", "");
        self
    }

    pub fn build(&mut self) -> Result<Url, URLError> {
        let url = Url::parse(&format!(
            "https://{}.{}/{}/data/query/{}?query={}",
            self.project_id, self.host, self.api_version, self.dataset, self.query
        ))
        .map_err(URLError::InvalidURL)?;
        Ok(url)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_simple() {
        let sanity_url = SanityURL::new()
            .project_id("abc123".to_string())
            .dataset("production".to_string())
            .api_version("v2022-03-07".to_string())
            .host("api.sanity.io".to_string())
            .query("".to_string())
            .build();
        match sanity_url {
            Ok(url) => {
                assert_eq!(
                    url.as_str(),
                    "https://abc123.api.sanity.io/v2022-03-07/data/query/production?query="
                );
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        };
    }

    #[test]
    fn query_test_one() {
        let query = r#"
        *[_id == "09139a58-311b-4779-8fa4-723f19242a8e"]{
            _id,
            _type,
            _createdAt,
            _updatedAt
        }"#;
        let sanity_url = SanityURL::new()
            .project_id("abc123".to_string())
            .dataset("production".to_string())
            .api_version("v2022-03-07".to_string())
            .host("api.sanity.io".to_string())
            .query(query.to_string())
            .build()
            .unwrap();
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2022-03-07/data/query/production?query=*[_id==%2209139a58-311b-4779-8fa4-723f19242a8e%22]{_id,_type,_createdAt,_updatedAt}"
        );
    }
}
