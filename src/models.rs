use super::schema::*;
use crate::*;
use serde::{Deserialize, Serialize};

// Post
#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "post"]
pub struct NewPost {
    pub body: String,
    pub user_id: Option<i32>,
}

// User
use argon2::{hash_encoded, Config};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use tower_cookies::Cookies;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    #[throws]
    pub fn hash_password(&mut self) {
        let salt = b"saltsalt";
        self.password = hash_encoded(self.password.as_bytes(), salt, &Config::default())?;
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for User {
    type Rejection = super::Error;

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
