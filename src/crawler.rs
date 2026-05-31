use crate::models::Page;
use std::error::Error;

pub async fn crawl(
    url: &str,
    depth: usize,
    simultaneous_requests: usize,
) -> Result<Vec<Page>, Box<dyn Error>> {
    Ok(vec![])
}
