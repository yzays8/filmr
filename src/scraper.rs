use std::{fs::File, io::Write, path::Path};

use anyhow::{Ok, Result, anyhow, bail};
use colored::Colorize;
use indicatif::ProgressBar;
use regex::Regex;
use reqwest::{StatusCode, blocking};
use scraper::{ElementRef, Html, Selector};
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

#[derive(Debug)]
pub struct Scraper {
    user_page_url: String,
}

impl Scraper {
    pub fn new(user_page_url: &str) -> Self {
        Self {
            user_page_url: user_page_url.to_string(),
        }
    }

    pub fn scrape(&self) -> Result<UserReviews> {
        let mut reviews = Vec::new();
        let mut is_first_page = true;
        let mut page_index = 1;
        let mut proc_url = self.user_page_url.clone();

        loop {
            let res = blocking::get(&proc_url)?;
            if res.status() == StatusCode::NOT_FOUND {
                if is_first_page {
                    bail!("User not found");
                } else {
                    // No more pages
                    break;
                }
            } else {
                is_first_page = false;
            }
            println!("Fetching reviews from {}...", proc_url.bright_cyan());

            let html = Html::parse_document(&res.text()?);
            let s = Selector::parse("div.p-contents-list div.c-content-card")
                .map_err(|e| anyhow!("Failed to parse selector {}", e))?;
            let reviews_in_page_iter = html.select(&s);

            let pb = ProgressBar::new(reviews_in_page_iter.clone().count() as u64);
            pb.set_style(
                indicatif::ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )?
                .progress_chars("##-"),
            );

            let reviews_in_page = reviews_in_page_iter
                .map(|elem| -> Result<UserReview> {
                    pb.inc(1);
                    let is_short_review = elem
                        .select(
                            &Selector::parse("span.c-content-card__readmore-review")
                                .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                        )
                        .next()
                        .is_none();
                    if is_short_review {
                        self.parse_short_review(elem)
                    } else {
                        self.parse_long_review(elem)
                    }
                })
                .collect::<Result<Vec<UserReview>>>()?;
            pb.finish_and_clear();
            println!("Done! {} reviews found.", reviews_in_page.len());

            reviews.extend(reviews_in_page);

            page_index += 1;
            proc_url = format!("{}?page={}", self.user_page_url, page_index);
        }

        Ok(UserReviews { reviews })
    }

    fn parse_short_review(&self, elem: ElementRef) -> Result<UserReview> {
        let regex_card = Regex::new(r"(.+)\((\d{4}).+\)")?;
        let regex_short_rev =
            Regex::new(r#"<p class="c-content-card__review"><span>(.*)</span></p>"#)?;

        let title_and_year = elem
            .select(
                &Selector::parse("h3.c-content-card__title")
                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
            )
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let captures = regex_card.captures(&title_and_year).unwrap();
        let title = captures.get(1).unwrap().as_str().to_string();
        let year = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let score = elem
            .select(
                &Selector::parse("div.c-rating__score")
                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
            )
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .parse::<f32>()
            .unwrap_or(0.0);
        let review = regex_short_rev
            .captures(
                &elem
                    .select(
                        &Selector::parse("p.c-content-card__review")
                            .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                    )
                    .next()
                    .unwrap()
                    .html(),
            )
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("<br>", "\n");
        Ok(UserReview {
            title,
            year,
            score,
            review,
        })
    }

    fn parse_long_review(&self, elem: ElementRef) -> Result<UserReview> {
        let regex_card = Regex::new(r"(.+)\((\d{4}).+\)")?;
        let regex_long_rev = Regex::new(r#"<div class="p-mark-review">(.+)</div>"#)?;

        let uri = elem
            .select(
                &Selector::parse("span.c-content-card__readmore-review a")
                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
            )
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        let document =
            Html::parse_document(&blocking::get(format!("https://filmarks.com{}", uri))?.text()?);
        let title_and_year = document
            .select(
                &Selector::parse("div.p-timeline-mark__title")
                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
            )
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let captures = regex_card.captures(&title_and_year).unwrap();
        let title = captures.get(1).unwrap().as_str().to_string();
        let year = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let score = document
            .select(
                &Selector::parse("div.c-rating__score")
                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
            )
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .parse::<f32>()
            .unwrap_or(0.0);
        let review = regex_long_rev
            .captures(
                &document
                    .select(
                        &Selector::parse("div.p-mark-review")
                            .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                    )
                    .next()
                    .unwrap()
                    .html(),
            )
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("<br>", "\n");
        Ok(UserReview {
            title,
            year,
            score,
            review,
        })
    }
}
