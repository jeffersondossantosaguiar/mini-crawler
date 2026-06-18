use crate::{error::CrawlerError, models::Page};

use reqwest::Client;
use scraper::{Html, Selector};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

pub async fn crawl(
    url: &str,
    max_depth: usize,
    simultaneous_requests: usize,
) -> Result<Vec<Page>, CrawlerError> {
    let client = Client::builder().user_agent("MiniCrawler").build()?;

    let queue: Arc<Mutex<Vec<(String, usize)>>> = Arc::new(Mutex::new(vec![(url.to_string(), 0)]));
    let visited: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let results: Arc<Mutex<Vec<Page>>> = Arc::new(Mutex::new(vec![]));
    let semaphore = Arc::new(Semaphore::new(simultaneous_requests));

    let mut handles = vec![];

    loop {
        let url = {
            let mut queue_lock = queue.lock().await;
            queue_lock.pop()
        };

        if url.is_none() {
            if semaphore.available_permits() == simultaneous_requests {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            continue;
        }

        if let Some((url, current_depth)) = url {
            if visited.lock().await.contains(&url) {
                continue;
            }

            if current_depth > max_depth {
                continue;
            }

            let client_clone = client.clone();

            let queue_clone = Arc::clone(&queue);
            let visited_clone = Arc::clone(&visited);
            let results_clone = Arc::clone(&results);
            let semaphore_clone = Arc::clone(&semaphore);

            let permit = semaphore_clone.acquire_owned().await?;

            handles.push(tokio::spawn(async move {
                if let Err(e) = process_url(
                    &url,
                    current_depth,
                    queue_clone,
                    visited_clone,
                    results_clone,
                    client_clone,
                )
                .await
                {
                    eprintln!("Error processing {}: {}", url, e);
                }
                drop(permit);
            }));
        };
    }

    Ok(results.lock().await.clone())
}

async fn process_url(
    url: &str,
    current_depth: usize,
    queue: Arc<Mutex<Vec<(String, usize)>>>,
    visited: Arc<Mutex<Vec<String>>>,
    results: Arc<Mutex<Vec<Page>>>,
    client: Client,
) -> Result<(), CrawlerError> {
    let response = client.get(url).send().await?;
    let html = response.text().await?;

    let links: Vec<String>;
    let title;

    {
        let document = Html::parse_document(&html);

        let title_selector = Selector::parse("title").unwrap();
        title = document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_else(|| "No Title".to_string());

        let links_selector = Selector::parse("a").unwrap();
        links = document
            .select(&links_selector)
            .filter_map(|e| e.value().attr("href"))
            .filter(|href| href.starts_with("http"))
            .map(|href| href.to_string())
            .collect();
    }

    visited.lock().await.push(url.to_string());

    let mut queue_lock = queue.lock().await;

    for link in &links {
        queue_lock.push((link.clone(), current_depth + 1));
    }

    results.lock().await.push(Page {
        url: url.to_string(),
        title,
        links,
    });

    Ok(())
}
