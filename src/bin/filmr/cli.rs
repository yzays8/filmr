use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "User ID to scrape reviews from")]
    pub user_id: String,

    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,

    #[arg(long, conflicts_with_all = ["drama", "anime"], help = "Retrieve movie reviews (default)")]
    pub movie: bool,

    #[arg(long, conflicts_with = "anime", help = "Retrieve drama reviews")]
    pub drama: bool,

    #[arg(long, help = "Retrieve anime reviews")]
    pub anime: bool,

    #[arg(short, long, default_value_t = FileType::Txt, value_name = "FORMAT", help = "Output format")]
    pub format: FileType,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FileType {
    Csv,
    Json,
    Txt,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Csv => write!(f, "csv"),
            FileType::Json => write!(f, "json"),
            FileType::Txt => write!(f, "txt"),
        }
    }
}
