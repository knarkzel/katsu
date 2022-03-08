// Mods
pub mod models;
pub mod schema;
pub mod template;

// Useful everywhere
#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;
pub use eyre::WrapErr;
pub use fehler::throws;
pub use sailfish::TemplateOnce;

// Custom error
use axum::{
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
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

// Database
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

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
