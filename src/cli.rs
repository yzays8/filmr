use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "User ID to scrape reviews from")]
    pub user_id: String,

    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,

    #[arg(long, conflicts_with_all = ["drama", "anime"], help = "Scrape movie reviews (default)")]
    pub movie: bool,

    #[arg(long, conflicts_with = "anime", help = "Scrape drama reviews")]
    pub drama: bool,

    #[arg(long, help = "Scrape anime reviews")]
    pub anime: bool,

    #[arg(short, long, help = "Output format (csv, json, txt). Default: txt")]
    pub format: Option<String>,
}
