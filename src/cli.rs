use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Username")]
    pub username: String,

    #[arg(long, help = "Scrape movie reviews")]
    pub movie: Option<bool>,

    #[arg(long, help = "Scrape drama reviews")]
    pub drama: Option<bool>,

    #[arg(long, help = "Scrape anime reviews")]
    pub anime: Option<bool>,

    #[arg(short, long, help = "Output format (json, csv)")]
    pub format: Option<String>,
}
