use bytes::Bytes;
use http::Request;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::Error;
use crate::response::{BoxBody, IntoResponse};

pub struct Json<T>(pub T);

pub type PathParams = HashMap<String, String>;

pub trait FromRequest: Sized {
    fn from_request(
        req: Request<Incoming>,
        params: &PathParams,
    ) -> impl std::future::Future<Output = Result<Self, Error>> + Send;
}

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned + Send> FromRequest for Json<T> {
    async fn from_request(req: Request<Incoming>, _params: &PathParams) -> Result<Self, Error> {
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

pub fn extract_path_params(pattern: &str, path: &str) -> Option<PathParams> {
    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (pattern_part, path_part) in pattern_parts.iter().zip(path_parts.iter()) {
        if let Some(param_name) = pattern_part.strip_prefix(':') {
            params.insert(param_name.to_string(), path_part.to_string());
        } else if pattern_part != path_part {
            return None;
        }
    }

    Some(params)
}
