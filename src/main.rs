use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use katsu::*;
use std::net::SocketAddr;

#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/create", post(create))
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

#[throws]
async fn create(
    Form(tweet): Form<models::NewTweet>,
    Extension(database): Extension<Database>,
) -> impl IntoResponse {
    use schema::tweet;
    diesel::insert_into(tweet::table)
        .values(tweet)
        .execute(&database.connection()?)?;
    Redirect::to("/".parse()?)
}
