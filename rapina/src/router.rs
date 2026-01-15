use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use http::{Method, Request, Response, StatusCode};
use hyper::body::Incoming;

use crate::response::{BoxBody, IntoResponse};

type BoxFuture = Pin<Box<dyn Future<Output = Response<BoxBody>> + Send>>;
type HandlerFn = Box<dyn Fn(Request<Incoming>) -> BoxFuture + Send + Sync>;

pub struct Router {
    routes: HashMap<(Method, String), HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route<F, Fut, Out>(mut self, method: Method, path: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        let handler = Box::new(move |req: Request<Incoming>| {
            let handler = handler.clone();
            Box::pin(async move {
                let output = handler(req).await;
                output.into_response()
            }) as BoxFuture
        });

        self.routes.insert((method, path.to_string()), handler);
        self
    }

    pub fn get<F, Fut, Out>(self, path: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        self.route(Method::GET, path, handler)
    }

    pub fn post<F, Fut, Out>(self, path: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        self.route(Method::POST, path, handler)
    }

    pub async fn handle(&self, req: Request<Incoming>) -> Response<BoxBody> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        match self.routes.get(&(method, path)) {
            Some(handler) => handler(req).await,
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
