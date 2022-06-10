use std::convert::Infallible;
use log::{error, info};

use hyper::{Body, Request, Response};

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Incoming request: {}", request.uri().path());
    Ok(Response::new("Hello, World!\n".into()))
}

#[sidevm::main]
async fn main() {
    sidevm::logger::Logger::with_max_level(log::Level::Trace).init();

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle))
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
