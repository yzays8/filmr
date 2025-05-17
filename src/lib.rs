#[deny(unsafe_code)]
mod anime;
mod app;
mod film;
mod scraper;
mod tv_series;

pub use app::App;
pub use scraper::{Config, FileType};
