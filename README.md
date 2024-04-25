# filmr

filmr is a scraping tool for [Filmarks](https://filmarks.com/). Enter a user ID and the user's previous reviews can be output in json/csv/txt format.

## Usage

```text
$ cargo run -- --help
Usage: filmr [OPTIONS] <USER_ID>

Arguments:
  <USER_ID>  User ID to scrape reviews from

Options:
  -o, --output <OUTPUT>  Output file
      --movie            Scrape movie reviews (default)
      --drama            Scrape drama reviews
      --anime            Scrape anime reviews
  -f, --format <FORMAT>  Output format (csv, json, txt). Default: txt
  -h, --help             Print help
  -V, --version          Print version
```

Example:

```sh
cargo run <USER_ID> -f csv --drama -o drama_reviews.csv
```
