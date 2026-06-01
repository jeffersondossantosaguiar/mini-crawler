use crate::models::Page;

use reqwest::get;
use scraper::{Html, Selector};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

pub async fn crawl(
    url: &str,
    max_depth: usize,
    simultaneous_requests: usize,
) -> Result<Vec<Page>, Box<dyn Error>> {
    let queue: Arc<Mutex<Vec<(String, usize)>>> = Arc::new(Mutex::new(vec![(url.to_string(), 0)]));
    let visited: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let results: Arc<Mutex<Vec<Page>>> = Arc::new(Mutex::new(vec![]));

    loop {
        let url = {
            let mut queue_lock = queue.lock().await;
            queue_lock.pop()
        };

        if url.is_none() {
            break;
        }

        if let Some((url, current_depth)) = url {
            if visited.lock().await.contains(&url) {
                continue;
            }

            if current_depth > max_depth {
                continue;
            }

            let response = get(&url).await?;
            let html = response.text().await?;

            let document = Html::parse_document(&html);

            let title_selector = Selector::parse("title").unwrap();
            let title = document
                .select(&title_selector)
                .next()
                .map(|e| e.inner_html())
                .unwrap_or_else(|| "No Title".to_string());

            let links_selector = Selector::parse("a").unwrap();
            let links: Vec<String> = document
                .select(&links_selector)
                .filter_map(|e| e.value().attr("href"))
                .filter(|href| href.starts_with("http"))
                .map(|href| href.to_string())
                .collect();

            visited.lock().await.push(url.clone());

            let mut queue_lock = queue.lock().await;

            for link in &links {
                queue_lock.push((link.clone(), current_depth + 1));
            }

            results.lock().await.push(Page {
                url: url.clone(),
                title,
                links,
            });
        }
    }

    Ok(results.lock().await.clone())
}
