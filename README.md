# filmr

filmr is a scraping tool to get the reviews of a specific user on [Filmarks](https://filmarks.com/). It supports `csv`, `json`, and `txt` as output formats.

## Usage

```text
$ cargo run -- --help
Usage: filmr [OPTIONS] <USER_ID>

Arguments:
  <USER_ID>  User ID to scrape reviews from

Options:
  -o, --output <OUTPUT>  Output file
      --movie            Retrieve movie reviews (default)
      --tvs              Retrieve TV series reviews
      --anime            Retrieve anime reviews
  -f, --format <FORMAT>  Output format [default: csv] [possible values: csv, json, txt]
  -h, --help             Print help
  -V, --version          Print version
```

### Example

```sh
cargo run <USER_ID> --anime -o anime_reviews.csv
```
