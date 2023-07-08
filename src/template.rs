use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub description: String,
    pub image: String,
    pub video: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}
