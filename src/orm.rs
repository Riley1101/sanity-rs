use super::client::SanityClient;
use super::error::RequestError;
use super::url::SanityURL;
use serde::de::DeserializeOwned;

/// A trait defining the interface for an Object-Relational Mapper (ORM).  This trait provides methods for interacting with a data source,
/// retrieving data in JSON format, and performing CRUD operations.
pub trait ORM {
    /// Serialized reterived data into JSON.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type to deserialize the JSON data into.  Must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// * `Result<T, RequestError>`: A Result containing the deserialized data or a `RequestError` if an error occurred.
    fn json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, RequestError>;

    /// Retrieves a single record from based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the record to retrieve.
    ///
    /// # Returns
    ///
    /// * `&mut SanityClient`: A mutable reference to the SanityClient.  This likely needs further clarification
    ///   depending on the actual implementation.  Consider returning a Result instead.
    fn get_by_id(&mut self, id: &str) -> &mut SanityClient;

    /// Retrieves multiple records based on their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids`: A slice of IDs of the records to retrieve.
    ///
    /// # Returns
    ///
    /// * `&mut SanityClient`: A mutable reference to the SanityClient. This likely needs further clarification
    ///   depending on the actual implementation. Consider returning a Result instead.
    fn get_by_ids(&mut self, ids: &[&str]) -> &mut SanityClient;

    /// Sends a request to the data source.
    ///
    /// # Returns
    ///
    /// * `impl Future<Output = Result<&mut Self, RequestError>>`: A future that resolves to a Result containing a mutable reference to `Self`
    ///   or a `RequestError` if an error occurred.
    fn send(&mut self) -> impl std::future::Future<Output = Result<&mut Self, RequestError>>;
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

