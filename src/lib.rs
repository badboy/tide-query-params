use tide::Context;

/// An extension to `Context`, providing query parameter extraction.
pub trait ExtractQueryParams {
    /// Extracts the query parameters from the URI.
    /// Returns `None` if no query parameters are available.
    fn query_params<T: serde::de::DeserializeOwned>(&mut self) -> Option<T>;
}

impl<AppData> ExtractQueryParams for Context<AppData> {
    fn query_params<T: serde::de::DeserializeOwned>(&mut self) -> Option<T> {
        let req = self.request();
        let uri = req.uri();
        let query = uri.query();
        query.and_then(|q| serde_qs::from_str(q).ok())
    }
}
