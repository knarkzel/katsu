use axum::{extract::Extension, routing::get, Router};
use katsu::*;
use std::net::SocketAddr;

#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(Database::new()?));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
}

#[throws]
async fn index(Extension(database): Extension<Database>) -> impl IntoResponse {
    use schema::tweet::dsl::*;
    let connection = database.connection()?;
    let tweets = tweet
        .load::<models::Tweet>(&connection)
        .wrap_err("Failed do load tweet")?;
    let output = template::Index { tweets }.render_once()?;
    Html(output)
}
