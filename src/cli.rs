use clap::Parser;

#[derive(Parser)]
#[command(name = "mini-crawler", version, about = "A web crawler")]
pub struct Input {
    #[arg(short, long, help = "Starting URL")]
    pub url: String,
    #[arg(short, long, help = "Crawling depth", default_value_t = 2)]
    pub depth: usize,
    #[arg(short, long, help = "Simultaneous requests", default_value_t = 5)]
    pub simultaneous_requests: usize,
}

impl Input {
    pub fn parse_args() -> Self {
        Input::parse()
    }
}
