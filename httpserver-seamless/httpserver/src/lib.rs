use futures::TryStreamExt;

use log::{error, info};
use std::io::{Error, ErrorKind};
use std::{convert::Infallible, sync::Arc};

use hyper::{Body, Request, Response};

use seamless::{
    api::{Api, RouteError},
    handler::{body::FromJson, response::ToJson},
};

struct SeamlessApi(Api);

impl SeamlessApi {
    async fn handle(&self, request: Request<Body>) -> http::Result<Response<Body>> {
        info!("Incoming request: {}", request.uri().path());
        let request = request.map(|body| {
            body.map_err(|_| Error::new(ErrorKind::Other, "Error reading request body"))
                .into_async_read()
        });
        let response = self.0.handle(request).await;
        info!("Outgoing response: {:?}", response);
        match response {
            Ok(response) => Ok(response.map(Into::into)),
            Err(RouteError::NotFound(_)) => Response::builder().status(404).body(Body::empty()),
            Err(RouteError::Err(err)) => Response::builder().status(err.code).body(Body::empty()),
        }
    }
}

fn create_api() -> Api {
    let mut api = Api::new();
    api.add("/api/echo")
        .description("Echoes back a JSON string")
        .handler(|body: FromJson<String>| ToJson(body.0));
    api.add("/api/reverse")
        .description("Reverse an array of numbers")
        .handler(|body: FromJson<Vec<usize>>| {
            ToJson(body.0.into_iter().rev().collect::<Vec<usize>>())
        });
    api
}

#[sidevm::main]
async fn main() {
    sidevm::logger::Logger::with_max_level(log::LevelFilter::Debug).init();

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        let api = Arc::new(SeamlessApi(create_api()));
        Ok::<_, Infallible>(hyper::service::service_fn(move |req| {
            let api = api.clone();
            async move { api.handle(req).await }
        }))
    });

    let address = "127.0.0.1:1999";
    info!("Listening on {}", address);

    let listener = sidevm::net::TcpListener::bind(address).await.unwrap();

    let server = hyper::Server::builder(listener)
        .executor(sidevm::exec::HyperExecutor)
        .serve(make_svc);
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
