use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{anime, movie, tv_series};

#[derive(Debug)]
pub struct Config {
    pub user_id: String,
    pub output: Option<String>,
    pub is_movie: bool,
    pub is_tv_series: bool,
    pub is_anime: bool,
    pub format: FileType,
}

#[derive(Debug, Clone, Copy)]
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

fn get_scraper(config: &Config) -> Box<dyn Scraper> {
    match (config.is_movie, config.is_tv_series, config.is_anime) {
        (_, false, false) => Box::new(movie::MovieScraper::new(&config.user_id)),
        (false, true, false) => Box::new(tv_series::TvSeriesScraper::new(&config.user_id)),
        (false, false, true) => Box::new(anime::AnimeScraper::new(&config.user_id)),
        _ => unreachable!(),
    }
}

pub fn run(config: &Config) -> Result<()> {
    let file_path = match &config.output {
        Some(path) => Path::new(path),
        None => match config.format {
            FileType::Csv => Path::new("reviews.csv"),
            FileType::Json => Path::new("reviews.json"),
            FileType::Txt => Path::new("reviews.txt"),
        },
    };
    file_path.try_exists()?;
    get_scraper(config)
        .scrape()?
        .export(config.format, file_path)?;
    Ok(())
}
