#![deny(unsafe_code)]

mod cli;

use clap::Parser as _;

use cli::{Args, FileType, MediaType};
use filmr::{self, App, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let media = match args.media {
        MediaType::Film => filmr::MediaType::Film,
        MediaType::Tvs => filmr::MediaType::Tvs,
        MediaType::Anime => filmr::MediaType::Anime,
    };
    let format = match args.format {
        FileType::Csv => filmr::FileType::Csv,
        FileType::Json => filmr::FileType::Json,
        FileType::Txt => filmr::FileType::Txt,
    };
    let config = Config {
        user_id: args.user_id,
        output: args.output,
        media,
        format,
        rate: args.rate,
    };

    App::new(config).run().await?;

    Ok(())
}
