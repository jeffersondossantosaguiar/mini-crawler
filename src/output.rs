use csv::Writer;
use serde_json::to_string_pretty;
use std::fs::write;

use crate::{cli::OutputFormat, error::CrawlerError, models::Page};

pub fn save(pages: Vec<Page>, format: OutputFormat) -> Result<(), CrawlerError> {
    match format {
        OutputFormat::Json => {
            let json = to_string_pretty(&pages)?;
            write("output.json", json)?;
        }
        OutputFormat::Csv => {
            let mut write = Writer::from_path("output.csv")?;
            write.write_record(["url", "title", "links"])?;
            for page in pages {
                write.serialize((&page.url, &page.title, &page.links.join(";")))?;
            }
            write.flush()?;
        }
    }
    Ok(())
}
