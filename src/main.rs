use axum::{response::IntoResponse, routing::get, Router};
use katsu::*;
use std::net::SocketAddr;

#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
}

async fn index() -> impl IntoResponse {
    template::Index {
        users: ["James", "Tom", "Marcus"],
    }
    .render()
}
