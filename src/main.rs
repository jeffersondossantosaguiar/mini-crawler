mod cli;
mod crawler;
mod error;
mod models;
mod output;

use crate::{cli::Input, crawler::crawl, output::save};

#[tokio::main]
async fn main() {
    let args = Input::parse_args();

    let pages = crawl(&args.url, args.depth, args.simultaneous_requests).await;

    match pages {
        Ok(pages) => match save(pages, args.output) {
            Ok(_) => println!("Crawling completed and results saved successfully."),
            Err(e) => eprintln!("Error saving results: {}", e),
        },
        Err(e) => eprintln!("Error during crawling: {}", e),
    }
}
