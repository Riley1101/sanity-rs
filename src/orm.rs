use super::client::SanityClient;
use super::error::RequestError;
use super::url::SanityURL;
use serde::de::DeserializeOwned;
use std::future::Future;

pub trait ORM {
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError>;
    fn get_by_id(&mut self, id: &str) -> &mut SanityClient;
    fn get_by_ids(&mut self, ids: &[&str]) -> &mut SanityClient;
    fn send(&mut self) -> impl Future<Output = Result<&mut Self, RequestError>>;
}

impl ORM for SanityClient {
    fn get_by_id(&mut self, id: &str) -> &mut SanityClient {
        let string = format!("*[_id == '{}'][0]", id);
        let query = &mut self.payload.query;
        SanityURL::query(query, string.as_str());
        self
    }

    fn get_by_ids(&mut self, ids: &[&str]) -> &mut SanityClient {
        let string = format!(r#"*[_id in ["{}"]]"#, ids.join("\",\""));
        let query = &mut self.payload.query;
        SanityURL::query(query, string.as_str());
        self
    }

    /// Parse the JSON response
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError> {
        let res = self.payload.query_result.as_ref().unwrap();
        let value: T = serde_json::from_str(res).map_err(RequestError::JsonParsingError)?;
        Ok(value)
    }

    async fn send(&mut self) -> Result<&mut Self, RequestError> {
        let query = &mut self.payload.query;
        let body = &self.payload.body;
        let url = format!("{}{}", query.as_str(), body.as_ref().unwrap());
        let v = self.client.get(url.as_str()).send();
        let v = v.await?.text().await?;
        self.payload.query_result = Some(v);
        Ok(self)
    }
}

