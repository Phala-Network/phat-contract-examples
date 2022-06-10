use std::{
    convert::Infallible,
    future::{ready, Ready},
    task::{Context, Poll},
};

use hyper::{body::HttpBody, service::Service};
use routerify::{RequestService, RequestServiceBuilder, Router};
use sidevm::net::AddrStream;

#[derive(Debug)]
pub struct RouterService<B, E> {
    builder: RequestServiceBuilder<B, E>,
}

impl<
        B: HttpBody + Send + Sync + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
    > RouterService<B, E>
{
    /// Creates a new service with the provided router and it's ready to be used with the hyper [`serve`](https://docs.rs/hyper/0.14.4/hyper/server/struct.Builder.html#method.serve)
    /// method.
    pub fn new(router: Router<B, E>) -> routerify::Result<RouterService<B, E>> {
        let builder = RequestServiceBuilder::new(router)?;
        Ok(RouterService { builder })
    }
}

impl<
        B: HttpBody + Send + Sync + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
    > Service<&AddrStream> for RouterService<B, E>
{
    type Response = RequestService<B, E>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &AddrStream) -> Self::Future {
        let req_service = self.builder.build(conn.remote_addr());
        ready(Ok(req_service))
    }
}
