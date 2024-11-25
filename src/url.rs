use crate::error::URLError;
use url::form_urlencoded;
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
        let uri = query.replace(" ", "").trim().replace("\n", "");
        let encoded_query = form_urlencoded::byte_serialize(uri.as_bytes()).collect::<String>();
        self.query = encoded_query;
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
            .build()
            .unwrap();
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2022-03-07/data/query/production?query="
        );
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
            "https://abc123.api.sanity.io/v2022-03-07/data/query/production?query=*%5B_id%3D%3D%2209139a58-311b-4779-8fa4-723f19242a8e%22%5D%7B_id%2C_type%2C_createdAt%2C_updatedAt%7D"
        );
    }

    #[test]
    fn query_with_filter() {
        let query = r#"
        *[type == "post" && published == true]{
            title,
            author,
            categories[]->title
        }"#;
        let sanity_url = SanityURL::new()
            .project_id("abc123".to_string())
            .dataset("blog".to_string())
            .api_version("v2023-01-01".to_string())
            .host("api.sanity.io".to_string())
            .query(query.to_string())
            .build()
            .unwrap();
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2023-01-01/data/query/blog?query=*%5Btype%3D%3D%22post%22%26%26published%3D%3Dtrue%5D%7Btitle%2Cauthor%2Ccategories%5B%5D-%3Etitle%7D"
        );
    }

    #[test]
    fn empty_query() {
        let sanity_url = SanityURL::new()
            .project_id("xyz456".to_string())
            .dataset("test".to_string())
            .api_version("v2023-05-01".to_string())
            .host("api.sanity.io".to_string())
            .query("".to_string())
            .build()
            .unwrap();
        assert_eq!(
            sanity_url.as_str(),
            "https://xyz456.api.sanity.io/v2023-05-01/data/query/test?query="
        );
    }

    #[test]
    fn query_with_special_characters() {
        let query = r#"
        *[name == "O'Reilly" && price < 100.0]{
            name,
            price
        }"#;
        let sanity_url = SanityURL::new()
            .project_id("abc123".to_string())
            .dataset("store".to_string())
            .api_version("v2023-05-01".to_string())
            .host("api.sanity.io".to_string())
            .query(query.to_string())
            .build()
            .unwrap();
        assert_eq!(
            sanity_url.as_str(),
            "https://abc123.api.sanity.io/v2023-05-01/data/query/store?query=*%5Bname%3D%3D%22O%27Reilly%22%26%26price%3C100.0%5D%7Bname%2Cprice%7D"
        );
    }
}
