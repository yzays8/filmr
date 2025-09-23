use std::time::Duration;

use reqwest::{Client, Response};
use tokio_utils::RateLimiter;

pub struct RateLimitedClient {
    client: Client,
    limitter: RateLimiter,
}

impl RateLimitedClient {
    pub fn with_rate(rate: Duration) -> Self {
        Self {
            client: Client::new(),
            limitter: RateLimiter::new(rate),
        }
    }

    pub async fn get(&self, url: &str) -> reqwest::Result<Response> {
        self.limitter
            .throttle(|| async {
                let response = self.client.get(url).send().await?;
                Ok::<Response, reqwest::Error>(response)
            })
            .await
    }
}

impl Default for RateLimitedClient {
    fn default() -> Self {
        Self::with_rate(Duration::from_secs(1))
    }
}
