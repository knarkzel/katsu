#[derive(Queryable)]
pub struct Tweet {
    pub id: i32,
    pub body: String,
}
