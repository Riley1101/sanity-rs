use crate::client::SanityClient;
use crate::error::RequestError;

pub trait ORM {
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
    // add code here
}
