use std::{fs::File, io::Write, path::Path};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub user_id: String,
    pub output: Option<String>,
    pub is_film: bool,
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
    pub fn export(&self, format: FileType, path: &Path) -> Result<()> {
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
