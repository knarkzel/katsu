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
        .route("/create", post(create_post))
        .route("/register", get(register).post(register_user))
        .route("/login", get(login).post(login_user))
        .layer(Extension(Database::new()?))
        .layer(CookieManagerLayer::new());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
}

#[throws]
async fn index(
    Extension(database): Extension<Database>,
    user: Option<models::User>,
) -> impl IntoResponse {
    use schema::post::dsl::*;
    let connection = database.connection()?;
    let posts = post
        .load::<models::Post>(&connection)
        .wrap_err("Failed do load post")?;
    Html(template::Index { user, posts }.render_once()?)
}

#[throws]
async fn register() -> impl IntoResponse {
    Html(template::Register.render_once()?)
}

#[throws]
async fn register_user(
    Form(mut register_user): Form<models::NewUser>,
    Extension(database): Extension<Database>,
) -> impl IntoResponse {
    // Hash password
    register_user.hash_password()?;

    // Insert user
    use schema::user;
    let connection = database.connection()?;
    diesel::insert_into(user::table)
        .values(register_user)
        .execute(&connection)?;
    Redirect::to("/login".parse()?)
}

#[throws]
async fn login() -> impl IntoResponse {
    Html(template::Login.render_once()?)
}

#[throws]
async fn login_user(
    Form(mut login_user): Form<models::NewUser>,
    cookies: Cookies,
    Extension(database): Extension<Database>,
) -> impl IntoResponse {
    // Make sure user is valid
    login_user.hash_password()?;
    use schema::user::dsl::*;
    let connection = database.connection()?;
    let users: Vec<models::User> = user
        .filter(
            username
                .eq(&login_user.username)
                .and(password.eq(&login_user.password)),
        )
        .load(&connection)?;

    // Login
    if users.len() > 0 {
        cookies.add(Cookie::new("User", serde_json::to_string(&users[0])?));
    } else {
        return Err(Error(eyre!("User not found")));
    }
    Ok(Redirect::to("/".parse()?))
}

#[throws]
async fn create_post(
    Form(mut post): Form<models::NewPost>,
    Extension(database): Extension<Database>,
    user: models::User,
) -> impl IntoResponse {
    post.user_id = Some(user.id);
    use schema::post;
    let connection = database.connection()?;
    diesel::insert_into(post::table)
        .values(post)
        .execute(&connection)?;
    Redirect::to("/".parse()?)
}
