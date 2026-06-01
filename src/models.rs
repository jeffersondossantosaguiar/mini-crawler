use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub links: Vec<String>,
}
