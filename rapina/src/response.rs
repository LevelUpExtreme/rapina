use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;

pub type BoxBody = Full<Bytes>;

pub trait IntoResponse {
    fn into_response(self) -> Response<BoxBody>;
}

impl IntoResponse for Response<BoxBody> {
    fn into_response(self) -> Response<BoxBody> {
        self
    }
}

impl IntoResponse for &str {
    fn into_response(self) -> Response<BoxBody> {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Full::new(Bytes::from(self.to_owned())))
            .unwrap()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response<BoxBody> {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Full::new(Bytes::from(self.to_owned())))
            .unwrap()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response<BoxBody> {
        Response::builder()
            .status(self)
            .body(Full::new(Bytes::new()))
            .unwrap()
    }
}

impl IntoResponse for (StatusCode, String) {
    fn into_response(self) -> Response<BoxBody> {
        Response::builder()
            .status(self.0)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Full::new(Bytes::from(self.1)))
            .unwrap()
    }
}
