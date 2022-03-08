use serde::Deserialize;

// Tweet
use super::schema::tweet;

#[derive(Debug, Queryable)]
pub struct Tweet {
    pub id: i32,
    pub body: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "tweet"]
pub struct NewTweet {
    pub body: String,
}
