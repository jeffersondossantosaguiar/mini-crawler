use serde::Serialize;

#[derive(Serialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub links: Vec<String>,
}
