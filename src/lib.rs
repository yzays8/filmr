#[deny(unsafe_code)]
mod app;
mod scraper;

pub use app::App;
pub use scraper::{Config, FileType};
