use axum::{
    routing::get,
    Router,
};
use tokio;

mod server;
mod handlers;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(handlers::index))
        .route("/askama", get(handlers::askama_handler))
        .route("/maud", get(handlers::maud_handler));

    server::Server::new(&"0.0.0.0:6969".to_string(), &router).run().await;
}
