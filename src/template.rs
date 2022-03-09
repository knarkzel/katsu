use crate::*;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub user: Option<models::User>,
    pub posts: Vec<models::Post>,
}

#[derive(TemplateOnce)]
#[template(path = "register.stpl")]
pub struct Register;

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
pub struct Login;
