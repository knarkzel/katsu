use axum::response::Html;
use sailfish::TemplateOnce;

pub trait Render {
    fn render(self) -> Html<String>;
}

impl<T: TemplateOnce> Render for T {
    fn render(self) -> Html<String> {
        Html(self.render_once().expect("Error occured when rendering"))
    }
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index<'a> {
    pub users: [&'a str; 3],
}
