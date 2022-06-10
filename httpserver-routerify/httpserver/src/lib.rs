use log::{error, info};
use std::convert::Infallible;

use hyper::{Body, Request, Response};

use routerify::prelude::*;
use routerify::Router;

use service::RouterService;

mod service;

// Define an app state to share it across the route handlers and middlewares.
struct State(u64);

// A handler for "/" page.
async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    log::info!("home_handler");
    // Access the app state.
    let state = req.data::<State>().unwrap();
    log::info!("State value: {}", state.0);

    Ok(Response::new(Body::from("Home page")))
}

// A handler for "/users/:userId" page.
async fn user_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let user_id = req.param("userId").unwrap();
    Ok(Response::new(Body::from(format!("Hello {}", user_id))))
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
fn router() -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(State(100))
        .get("/", home_handler)
        .get("/users/:userId", user_handler)
        .build()
        .unwrap()
}

#[sidevm::main]
async fn main() {
    sidevm::logger::Logger::with_max_level(log::Level::Trace).init();
    sidevm::ocall::enable_ocall_trace(true).unwrap();

    let router = router();

    let service = RouterService::new(router).unwrap();

    let address = "127.0.0.1:1999";
    info!("Listening on {}", address);

    let listener = sidevm::net::TcpListener::bind(address).await.unwrap();

    let server = hyper::Server::builder(listener.into_addr_incoming())
        .executor(sidevm::exec::HyperExecutor)
        .serve(service);
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
