use askama::Template;

#[derive(Debug, Clone)]
pub struct Thread {
    pub title: String,
    pub url: String,
    pub description: String,
    pub image: String,
}

impl Thread {
    pub fn parse(meta: Vec<(&str, &str)>) -> Self {
        let mut title = String::new();
        let mut url = String::new();
        let mut description = String::new();
        let mut image = String::new();

        for (key, value) in meta {
            match key {
                "og:title" => title = value.to_string(),
                "og:url" => url = value.to_string(),
                "og:description" => description = value.to_string(),
                "og:image" => image = value.to_string(),
                _ => (),
            }
        }

        Self {
            title,
            url,
            description,
            image,
        }
    }
}

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
