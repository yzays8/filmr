use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{bail, Ok, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::cli::Args;
use crate::movie;

#[derive(Debug)]
pub enum FileType {
    Csv,
    Json,
    Txt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReview {
    pub title: String,
    pub year: i32,
    pub score: f32,
    pub review: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReviews {
    pub reviews: Vec<UserReview>,
}

impl UserReviews {
    fn export(&self, format: FileType, path: &Path) -> Result<()> {
        match format {
            FileType::Csv => {
                let mut writer = csv::Writer::from_path(path)?;
                for review in &self.reviews {
                    writer.serialize(review)?;
                }
                writer.flush()?;
            }
            FileType::Json => {
                let json = serde_json::to_string_pretty(&self)?;
                File::create(path)?.write_all(json.as_bytes())?;
            }
            FileType::Txt => {
                let mut file = File::create(path)?;
                for review in &self.reviews {
                    writeln!(
                        file,
                        "Title: {}\nYear: {}\nScore: {}\nReview:\n{}\n\n",
                        review.title, review.year, review.score, review.review
                    )?;
                }
            }
        }
        Ok(())
    }
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
    let args = Args::parse();
    let file_type = match &args.format {
        Some(format) => match format.to_lowercase().as_str() {
            "csv" => FileType::Csv,
            "json" => FileType::Json,
            "txt" => FileType::Txt,
            _ => bail!("Invalid format"),
        },
        None => FileType::Txt,
    };
    let file_path = match &args.output {
        Some(path) => Path::new(path),
        None => match file_type {
            FileType::Csv => Path::new("reviews.csv"),
            FileType::Json => Path::new("reviews.json"),
            FileType::Txt => Path::new("reviews.txt"),
        },
    };
    file_path.try_exists()?;
    get_scraper(&args).scrape()?.export(file_type, file_path)?;
    Ok(())
}
