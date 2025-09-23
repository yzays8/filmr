#![deny(unsafe_code)]

mod cli;

use clap::Parser as _;
use cli::{Args, FileType};

use filmr::{self, App, Config};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let format = match args.format {
        FileType::Csv => filmr::FileType::Csv,
        FileType::Json => filmr::FileType::Json,
        FileType::Txt => filmr::FileType::Txt,
    };
    let config = Config {
        user_id: args.user_id,
        output: args.output,
        is_film: args.film,
        is_tv_series: args.tvs,
        is_anime: args.anime,
        format,
        rate: args.rate,
    };

    if let Err(e) = App::new(config).run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
