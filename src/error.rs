use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Semaphore error: {0}")]
    Semaphore(#[from] tokio::sync::AcquireError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Writer error: {0}")]
    Writer(#[from] csv::Error),
}
