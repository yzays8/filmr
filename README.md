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
  -m, --media <MEDIA>    Type of media to retrieve [default: film] [possible values: film, tvs, anime]
  -f, --format <FORMAT>  Output format [default: csv] [possible values: csv, json, txt]
  -r, --rate <RATE>      Delay between requests (e.g. '500ms', '2s') [default: 1s]
  -h, --help             Print help
  -V, --version          Print version
```

### Example

```sh
cargo run <USER_ID> -m anime -o anime_reviews.csv
```
