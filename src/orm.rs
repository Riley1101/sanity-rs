use serde::de::DeserializeOwned;

use crate::client::SanityClient;
use crate::error::RequestError;

pub trait ORM {
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError>;
    fn get_by_id<T>(&self, id: &str) -> Result<T, RequestError>;
    fn get_documents<T>(
        &self,
        query: Vec<String>,
        document_ids: &[&str],
    ) -> Result<T, RequestError>;
}

impl ORM for SanityClient {
    fn get_by_id<T>(&self, id: &str) -> Result<T, RequestError> {
        todo!()
    }

    fn get_documents<T>(
        &self,
        query: Vec<String>,
        document_ids: &[&str],
    ) -> Result<T, RequestError> {
        todo!()
    }

    /// Parse the JSON response
    fn json<T: DeserializeOwned>(&mut self) -> Result<T, RequestError> {
        let res = self.payload.query_result.as_ref().unwrap();
        let value: T = serde_json::from_str(res).map_err(RequestError::JsonParsingError)?;
        Ok(value)
    }
    // add code here
}
