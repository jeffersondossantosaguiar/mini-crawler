# mini-crawler

A fast, concurrent web crawler written in Rust. Give it a URL and it will crawl the web, extracting page titles and links up to a configurable depth — saving results as JSON or CSV.

## Features

- Concurrent crawling with configurable simultaneous requests
- Configurable crawl depth
- Output as JSON or CSV
- Duplicate URL detection
- Custom User-Agent header

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or higher

### Build from source

```bash
git clone https://github.com/yourusername/mini-crawler
cd mini-crawler
cargo build --release
```

The binary will be available at `./target/release/mini-crawler`.

## Usage

```bash
mini-crawler --url <URL> [OPTIONS]
```

### Options

| Option                    | Short | Default  | Description                     |
| ------------------------- | ----- | -------- | ------------------------------- |
| `--url`                   | `-u`  | required | Starting URL to crawl           |
| `--depth`                 | `-d`  | `2`      | Maximum crawl depth             |
| `--simultaneous-requests` | `-s`  | `5`      | Number of concurrent requests   |
| `--output`                | `-o`  | `json`   | Output format (`json` or `csv`) |

### Examples

Crawl a website with default settings:

```bash
mini-crawler --url https://example.com
```

Crawl up to depth 3 with 10 simultaneous requests:

```bash
mini-crawler --url https://example.com --depth 3 --simultaneous-requests 10
```

Save results as CSV:

```bash
mini-crawler --url https://example.com --output csv
```

## Output

Results are saved to `output.json` or `output.csv` in the current directory.

### JSON output

```json
[
  {
    "url": "https://example.com",
    "title": "Example Domain",
    "links": ["https://iana.org/domains/example"]
  }
]
```

### CSV output

```
url,title,links
https://example.com,Example Domain,https://iana.org/domains/example
```

Multiple links in CSV are separated by `;`.

## How it works

1. Starts with the provided URL at depth `0`
2. Fetches the page and extracts the `<title>` and all `<a href>` links
3. Adds new links to the queue at `depth + 1`
4. Skips already-visited URLs and URLs beyond `max_depth`
5. Repeats concurrently until the queue is empty

## License

MIT
