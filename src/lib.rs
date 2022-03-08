// Mods
pub mod models;
pub mod schema;
pub mod template;

// Anyhow-like
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// .render() which always unwrap()s
pub use template::Render;

// Diesel
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub use diesel::prelude::*;

// Database
embed_migrations!();

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

#[derive(Clone)]
pub struct Database(Pool<ConnectionManager<SqliteConnection>>);

impl Database {
    pub fn new() -> Result<Self> {
        dotenv::dotenv()?;
        let url = std::env::var("DATABASE_URL")?;
        let manager = ConnectionManager::<SqliteConnection>::new(url);
        let pool = Pool::new(manager)?;
        let database = Self(pool);
        embedded_migrations::run_with_output(&database.connection()?, &mut std::io::stdout())?;
        Ok(database)
    }

    pub fn connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        Ok(self.0.get()?)
    }
}
