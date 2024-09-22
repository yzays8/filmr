mod cli;

use clap::Parser as _;
use cli::Args;

use filmr::Config;

fn main() {
    let args = Args::parse();
    let config = Config {
        user_id: args.user_id,
        output: args.output,
        is_movie: args.movie,
        is_drama: args.drama,
        is_anime: args.anime,
        format: args.format,
    };

    if let Err(e) = filmr::run(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
