pub mod regex_scraper;
pub mod email_scraper;

use std::str::FromStr;

pub enum Scrapers {
    email(email_scraper::EmailScraper)
}

impl FromStr for Scrapers {
    type Err = ();

    fn from_str(s: &str) -> Result<Scrapers, ()> {
        match s {
            "email" => Ok(self::Scrapers::email),
            _ => Err(()),
        }
    }
}
