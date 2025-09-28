use std::{fs::OpenOptions, path::Path};

use crate::{
    client::RateLimitedClient,
    error::Result,
    scraper::{Config, FileType, MediaType, Scraper},
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
        match self.config.media {
            MediaType::Film => {
                Scraper::new(&format!("{USER_BASE_URL}{}", self.config.user_id), client)
            }
            MediaType::Tvs => Scraper::new(
                &format!("{USER_BASE_URL}{}/marks/dramas", self.config.user_id),
                client,
            ),
            MediaType::Anime => Scraper::new(
                &format!("{USER_BASE_URL}{}/marks/animes", self.config.user_id),
                client,
            ),
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

        // Check if the file can be created.
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)?;

        self.get_scraper()
            .scrape()
            .await?
            .export(self.config.format, file_path)?;

        Ok(())
    }
}
