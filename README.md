# filmr

`filmr` is a scraping tool to get the reviews of a specific user on [Filmarks](https://filmarks.com/). It supports `csv`, `json`, and `txt` as output formats.

## Usage

```text
$ cargo run -- --help
Usage: filmr [OPTIONS] <USER_ID>

Arguments:
  <USER_ID>  User ID to scrape reviews from

Options:
  -o, --output <OUTPUT>  Output file
      --movie            Retrieve movie reviews (default)
      --drama            Retrieve drama reviews
      --anime            Retrieve anime reviews
  -f, --format <FORMAT>  Output format (csv, json, txt). Default: txt
  -h, --help             Print help
  -V, --version          Print version
```

Example:

```sh
cargo run <USER_ID> -f csv --drama -o drama_reviews.csv
```
