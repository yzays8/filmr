use std::path::Path;

use anyhow::Result;

use crate::scraper::{Config, FileType, Scraper};

const USER_BASE_URL: &str = "https://filmarks.com/users/";

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn get_scraper(&self) -> Scraper {
        match (
            self.config.is_film,
            self.config.is_tv_series,
            self.config.is_anime,
        ) {
            (_, false, false) => Scraper::new(&format!("{USER_BASE_URL}{}", self.config.user_id)),
            (false, true, false) => Scraper::new(&format!(
                "{USER_BASE_URL}{}/marks/dramas",
                self.config.user_id
            )),
            (false, false, true) => Scraper::new(&format!(
                "{USER_BASE_URL}{}/marks/animes",
                self.config.user_id
            )),
            _ => unreachable!(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let file_path = match &self.config.output {
            Some(path) => Path::new(path),
            None => match self.config.format {
                FileType::Csv => Path::new("reviews.csv"),
                FileType::Json => Path::new("reviews.json"),
                FileType::Txt => Path::new("reviews.txt"),
            },
        };
        file_path.try_exists()?;

        self.get_scraper()
            .scrape()?
            .export(self.config.format, file_path)?;

        Ok(())
    }
}
