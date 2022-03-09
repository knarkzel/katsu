// Mods
pub mod models;
pub mod schema;
pub mod template;

// Useful everywhere
#[macro_use]
extern crate diesel;

use async_trait::async_trait;
pub use diesel::prelude::*;
pub use eyre::{eyre, WrapErr};
pub use fehler::throws;
pub use sailfish::TemplateOnce;

// Custom error
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct Error(pub eyre::Report);

impl<E: Into<eyre::Report>> From<E> for Error {
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0)).into_response()
    }
}

// Database
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

#[derive(Clone)]
pub struct Database(Pool<ConnectionManager<SqliteConnection>>);

impl Database {
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv()?;
        let url = std::env::var("DATABASE_URL")?;
        let manager = ConnectionManager::<SqliteConnection>::new(url);
        let pool = Pool::new(manager)?;
        let database = Self(pool);
        embedded_migrations::run_with_output(&database.connection()?, &mut std::io::stdout())?;
        Ok(database)
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, Error> {
        Ok(self.0.get()?)
    }
}

// User
#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl<B: Send> FromRequest<B> for User {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookies = req.extensions().unwrap().get::<Cookies>().cloned().unwrap();
        match cookies.get("User") {
            Some(json) => {
                let user = serde_json::from_str(json.value())?;
                Ok(user)
            }
            None => Err(Error(eyre!("User not found"))),
        }
    }
}
