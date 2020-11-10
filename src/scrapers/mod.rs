pub mod regex_scraper;
pub mod email_scraper;

use std::str::FromStr;

pub enum Scrapers {
    EmailScraper
}

pub trait Execute {
    fn execute(content: String) -> Vec<String> {
        let mut results = Vec::new();

        results
    }
}

impl FromStr for Scrapers {
    type Err = ();

    fn from_str(s: &str) -> Result<Scrapers, ()> {
        match s {
            "email" => Ok(Scrapers::EmailScraper),
            _ => Err(()),
        }
    }
}
