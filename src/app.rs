use std::path::Path;

use anyhow::Result;

use crate::{
    anime::AnimeScraper,
    film::FilmScraper,
    scraper::{Config, FileType, Scraper},
    tv_series::TvSeriesScraper,
};

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn get_scraper(&self) -> Box<dyn Scraper> {
        match (
            self.config.is_film,
            self.config.is_tv_series,
            self.config.is_anime,
        ) {
            (_, false, false) => Box::new(FilmScraper::new(&self.config.user_id)),
            (false, true, false) => Box::new(TvSeriesScraper::new(&self.config.user_id)),
            (false, false, true) => Box::new(AnimeScraper::new(&self.config.user_id)),
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
