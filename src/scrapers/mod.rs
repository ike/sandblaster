pub mod regex_scraper;
pub mod email_scraper;

use std::str::FromStr;

pub enum ScraperType {
    EmailScraper,
    PhoneScraper
}

// This will probably come in handy later
//pub struct Scrapers {
//  scraper_type: ScraperType
//}

pub trait Execute {
    fn execute<'a>(&'a self, content: &'a String) -> Vec<String>;
}

impl FromStr for ScraperType {
    type Err = ();

    fn from_str(s: &str) -> Result<ScraperType, ()> {
        match s {
            "email" => Ok(ScraperType::EmailScraper),
            "phone" => Ok(ScraperType::PhoneScraper),
            _ => Err(()),
        }
    }
}
