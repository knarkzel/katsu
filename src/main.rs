use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use katsu::*;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/user", get(user))
        .route("/create", post(create))
        .layer(Extension(Database::new()?))
        .layer(CookieManagerLayer::new());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
}

#[throws]
async fn index(Extension(database): Extension<Database>, cookies: Cookies) -> impl IntoResponse {
    // Login
    let user = User {
        username: "Yo".to_string(),
        password: "Lol".to_string(),
    };
    cookies.add(Cookie::new("User", serde_json::to_string(&user)?));

    use schema::tweet::dsl::*;
    let connection = database.connection()?;
    let tweets = tweet
        .load::<models::Tweet>(&connection)
        .wrap_err("Failed do load tweet")?;
    let output = template::Index { tweets }.render_once()?;
    Html(output)
}

#[throws]
async fn user(user: User) -> impl IntoResponse {
    Html(format!("{} {}", user.username, user.password))
}

#[throws]
async fn create(
    Form(tweet): Form<models::NewTweet>,
    Extension(database): Extension<Database>,
) -> impl IntoResponse {
    use schema::tweet;
    let connection = database.connection()?;
    diesel::insert_into(tweet::table)
        .values(tweet)
        .execute(&connection)?;
    Redirect::to("/".parse()?)
}
