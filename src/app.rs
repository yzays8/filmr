use std::path::Path;

use crate::{
    client::RateLimitedClient,
    error::Result,
    scraper::{Config, FileType, Scraper},
};

// ref: https://filmarks.com/robots.txt
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
        let client = RateLimitedClient::with_rate(self.config.rate.into());
        match (
            self.config.is_film,
            self.config.is_tv_series,
            self.config.is_anime,
        ) {
            (_, false, false) => {
                Scraper::new(&format!("{USER_BASE_URL}{}", self.config.user_id), client)
            }
            (false, true, false) => Scraper::new(
                &format!("{USER_BASE_URL}{}/marks/dramas", self.config.user_id),
                client,
            ),
            (false, false, true) => Scraper::new(
                &format!("{USER_BASE_URL}{}/marks/animes", self.config.user_id),
                client,
            ),
            _ => unreachable!(),
        }
    }

    pub async fn run(&self) -> Result<()> {
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
            .scrape()
            .await?
            .export(self.config.format, file_path)?;

        Ok(())
    }
}
