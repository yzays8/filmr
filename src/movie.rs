use anyhow::{anyhow, bail, Ok, Result};
use regex::Regex;
use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};

use crate::scraper::{Scraper, UserReview, UserReviews};

#[derive(Debug)]
pub struct MovieScraper {
    username: String,
}

impl MovieScraper {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

impl Scraper for MovieScraper {
    fn scrape(&self) -> Result<UserReviews> {
        let mut reviews: Vec<UserReview> = Vec::new();
        let mut page = format!("https://filmarks.com/users/{}", self.username);
        let mut is_first_page = true;
        let mut page_index = 1;

        loop {
            println!("Fetching page {}", page);
            let res = blocking::get(&page)?;
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

            let reviews_in_page = Html::parse_document(&res.text()?)
                .select(
                    // div element of class "c-content-card" within a div element of class "p-contents-list"
                    &Selector::parse("div.p-contents-list div.c-content-card")
                        .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                )
                .map(|elem| -> Result<UserReview> {
                    if elem
                        .select(
                            &Selector::parse("span.c-content-card__readmore-review")
                                .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                        )
                        .next()
                        .is_none()
                    {
                        // Short review
                        let title_and_year = elem
                            .select(
                                &Selector::parse("h3.c-content-card__title")
                                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                            )
                            .next()
                            .unwrap()
                            .text()
                            .collect::<String>();
                        let captures = Regex::new(r"(.+)\((\d{4}).+\)")?
                            .captures(&title_and_year)
                            .unwrap();
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
                        let review = Regex::new(
                            r#"<p class="c-content-card__review"><span>(.*)</span></p>"#,
                        )?
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
                    } else {
                        // Long review
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
                        let document = Html::parse_document(
                            &blocking::get(format!("https://filmarks.com{}", uri))?.text()?,
                        );
                        let title_and_year = document
                            .select(
                                &Selector::parse("div.p-timeline-mark__title")
                                    .map_err(|e| anyhow!("Failed to parse selector {}", e))?,
                            )
                            .next()
                            .unwrap()
                            .text()
                            .collect::<String>();
                        let captures = Regex::new(r"(.+)\((\d{4}).+\)")
                            .unwrap()
                            .captures(&title_and_year)
                            .unwrap();
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
                        let review =
                            Regex::new(r#"<div class="p-mark__review">(.+)</div>"#)?
                                .captures(
                                    &document
                                        .select(&Selector::parse("div.p-mark__review").map_err(
                                            |e| anyhow!("Failed to parse selector {}", e),
                                        )?)
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
                })
                .collect::<Result<Vec<UserReview>>>()?;

            reviews.extend(reviews_in_page);

            // Get the next page of reviews.
            page_index += 1;
            page = format!(
                "https://filmarks.com/users/{}?page={}",
                self.username, page_index
            );
        }

        Ok(UserReviews { reviews })
    }
}
