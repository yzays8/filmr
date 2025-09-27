use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("{0}")]
    SelectorParse(String),
    #[error(transparent)]
    TemplateParse(#[from] indicatif::style::TemplateError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("user not found")]
    UserNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<scraper::error::SelectorErrorKind<'static>> for Error {
    fn from(e: scraper::error::SelectorErrorKind<'static>) -> Self {
        Error::SelectorParse(e.to_string())
    }
}
