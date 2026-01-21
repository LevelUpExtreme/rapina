use std::net::SocketAddr;

use crate::middleware::{Middleware, MiddlewareStack};
use crate::router::Router;
use crate::server::serve;
use crate::state::AppState;

pub struct Rapina {
    router: Router,
    state: AppState,
    middlewares: MiddlewareStack,
}

impl Rapina {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            state: AppState::new(),
            middlewares: MiddlewareStack::new(),
        }
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub fn state<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.state = self.state.with(value);
        self
    }

    pub fn middleware<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.add(middleware);
        self
    }

    pub async fn listen(self, addr: &str) -> std::io::Result<()> {
        let addr: SocketAddr = addr.parse().expect("invalid address");
        serve(self.router, self.state, self.middlewares, addr).await
    }
}

impl Default for Rapina {
    fn default() -> Self {
        Self::new()
    }
}
