pub mod regex_scraper;
pub mod email_scraper;

use std::str::FromStr;

pub enum Scrapers {
    EmailScraper
}

impl Scrapers {
    pub fn execute<'a>(&'a self, content: &'a String) -> Vec<String> {
        let c: String = content.clone();
        let results: Vec<String> = vec![c];

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
