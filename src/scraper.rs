use std::{fs::File, io::Write, path::Path, sync::Arc};

use anyhow::{Ok, Result, anyhow, bail};
use colored::Colorize;
use futures::future::join_all;
use humantime::Duration;
use indicatif::ProgressBar;
use regex::Regex;
use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::client::RateLimitedClient;

#[derive(Debug)]
pub struct Config {
    pub user_id: String,
    pub output: Option<String>,
    pub is_film: bool,
    pub is_tv_series: bool,
    pub is_anime: bool,
    pub format: FileType,
    pub rate: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Csv,
    Json,
    Txt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

pub struct Scraper {
    user_page_url: String,
    client: RateLimitedClient,
}

impl Scraper {
    pub fn new(user_page_url: &str, client: RateLimitedClient) -> Self {
        Self {
            user_page_url: user_page_url.to_string(),
            client,
        }
    }

    pub async fn scrape(&self) -> Result<UserReviews> {
        let mut reviews = Vec::new();
        let mut is_first_page = true;
        let mut page_index = 1;
        let mut proc_url = self.user_page_url.clone();

        loop {
            let res = self.client.get(&proc_url).await?;
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

            let html = Html::parse_document(&res.text().await?);
            let s = Selector::parse("div.p-contents-list div.c-content-card")
                .map_err(|e| anyhow!("Failed to parse selector {}", e))?;
            let review_elems = html.select(&s);
            let review_count = review_elems.clone().count();
            let pb = Arc::new(ProgressBar::new(review_count as u64));
            pb.set_style(
                indicatif::ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )?
                .progress_chars("##-"),
            );

            let mut review_slots = vec![None; review_count];
            let mut long_tasks = Vec::new();
            for (i, review_elem) in review_elems.enumerate() {
                let is_short_review = review_elem
                    .select(
                        &Selector::parse("span.c-content-card__readmore-review")
                            .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                    )
                    .next()
                    .is_none();

                if is_short_review {
                    review_slots[i] = Some(self.parse_short_review(review_elem)?);
                    pb.inc(1);
                } else {
                    let uri = review_elem
                        .select(
                            &Selector::parse("span.c-content-card__readmore-review a")
                                .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                        )
                        .next()
                        .unwrap()
                        .value()
                        .attr("href")
                        .unwrap();

                    let pb = Arc::clone(&pb);
                    long_tasks.push(async move {
                        let doc = Html::parse_document(
                            &self
                                .client
                                .get(&format!("https://filmarks.com{}", uri))
                                .await?
                                .text()
                                .await?,
                        );
                        let r = self.parse_long_review(&doc)?;
                        pb.inc(1);
                        Ok((i, r))
                    })
                }
            }

            let long_results = join_all(long_tasks).await;
            for res in long_results {
                let (i, review) = res?;
                review_slots[i] = Some(review);
            }

            pb.finish_and_clear();

            let reviews_in_page = review_slots
                .into_iter()
                .map(|r| r.unwrap())
                .collect::<Vec<_>>();
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

    fn parse_long_review(&self, html: &Html) -> Result<UserReview> {
        let regex_card = Regex::new(r"(.+)\((\d{4}).+\)")?;
        let regex_long_rev = Regex::new(r#"<div class="p-mark-review">(.+)</div>"#)?;

        let title_and_year = html
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
        let score = html
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
                &html
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
