use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Json,
    Csv,
}

#[derive(Parser)]
#[command(name = "mini-crawler", version, about = "A web crawler")]
pub struct Input {
    #[arg(short, long, help = "Starting URL")]
    pub url: String,
    #[arg(short, long, help = "Crawling depth", default_value_t = 2)]
    pub depth: usize,
    #[arg(short, long, help = "Simultaneous requests", default_value_t = 5)]
    pub simultaneous_requests: usize,
    #[arg(
        short,
        long,
        help = "Output format",
        value_enum,
        default_value = "json"
    )]
    pub output: OutputFormat,
}

impl Input {
    pub fn parse_args() -> Self {
        Input::parse()
    }
}
