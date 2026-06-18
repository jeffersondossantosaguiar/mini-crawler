use std::{fs::write, io};

use crate::{cli::OutputFormat, models::Page};

pub fn save(pages: Vec<Page>, format: OutputFormat) -> Result<(), io::Error> {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&pages)?;
            write("output.json", json)?;
        }
        OutputFormat::Csv => {
            let mut write = csv::Writer::from_path("output.csv")?;
            write.write_record(["url", "title", "links"])?;
            for page in pages {
                write.serialize((&page.url, &page.title, &page.links.join(";")))?;
            }
            write.flush()?;
        }
    }
    Ok(())
}
