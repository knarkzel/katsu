use axum::{extract::Extension, response::IntoResponse, routing::get, Router};
use katsu::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // Database
    let database = Database::new()?;

    // Axum
    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(database));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    // Ugly return type
    Ok(())
}

async fn index(Extension(database): Extension<Database>) -> impl IntoResponse {
    use schema::tweet::dsl::*;
    let connection = database.connection().unwrap();
    let tweets = tweet.load::<models::Tweet>(&connection).unwrap();
    template::Index { tweets }.render()
}
