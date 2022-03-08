use crate::*;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub tweets: Vec<models::Tweet>,
}
