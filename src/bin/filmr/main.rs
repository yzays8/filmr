mod cli;

use clap::Parser as _;
use cli::{Args, FileType};

use filmr::{self, run, Config};

fn main() {
    let args = Args::parse();
    let format = match args.format {
        FileType::Csv => filmr::FileType::Csv,
        FileType::Json => filmr::FileType::Json,
        FileType::Txt => filmr::FileType::Txt,
    };
    let config = Config {
        user_id: args.user_id,
        output: args.output,
        is_movie: args.movie,
        is_tv_series: args.tvs,
        is_anime: args.anime,
        format,
    };

    if let Err(e) = run(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
