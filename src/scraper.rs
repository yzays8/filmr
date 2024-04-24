use anyhow::{Ok, Result};
use clap::Parser;

use crate::cli::Args;
use crate::movie;

#[derive(Debug)]
pub struct UserReview {
    pub title: String,
    pub year: i32,
    pub score: f32,
    pub review: String,
}

#[derive(Debug)]
pub struct UserReviews {
    pub reviews: Vec<UserReview>,
}

pub trait Scraper {
    fn scrape(&self) -> Result<UserReviews>;
}

fn get_scraper(args: &Args) -> impl Scraper {
    match (args.movie, args.drama, args.anime) {
        (_, false, false) => movie::MovieScraper::new(&args.username),
        (false, true, false) => todo!(),
        (false, false, true) => todo!(),
        _ => unreachable!(),
    }
}

pub fn run() -> Result<()> {
    let reviews = get_scraper(&Args::parse()).scrape()?;
    println!("{:#?}", reviews);
    Ok(())
}
