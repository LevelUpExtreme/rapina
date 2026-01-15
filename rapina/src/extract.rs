use bytes::Bytes;
use http::Request;
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::response::{BoxBody, IntoResponse};

pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> Json<T> {
    pub async fn from_request(req: Request<hyper::body::Incoming>) -> Result<Self, Error> {
        let body = req.into_body();
        let bytes = body
            .collect()
            .await
            .map_err(|_| Error::bad_request("failed to read body"))?
            .to_bytes();

        let value: T = serde_json::from_slice(&bytes)
            .map_err(|e| Error::bad_request(format!("invalid JSON: {}", e)))?;

        Ok(Json(value))
    }
}

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> http::Response<BoxBody> {
        let body = serde_json::to_vec(&self.0).unwrap_or_default();
        http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(http_body_util::Full::new(Bytes::from(body)))
            .unwrap()
    }
}
