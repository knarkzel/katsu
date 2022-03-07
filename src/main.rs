use axum::{routing::get, Router};
use fehler::throws;
use std::net::SocketAddr;
type Error = anyhow::Error;

#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
}

async fn root() -> &'static str {
    "Hello, World!"
}
