extern crate regex;
use scrapers;
use scrapers::regex_scraper;
use scrapers::regex_scraper::Capture;

pub struct EmailScraper { }

impl regex_scraper::Capture for EmailScraper {
    fn expression(&self) -> regex::Regex {
        let regex = regex::Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?");
        regex.unwrap()
    }
}

impl EmailScraper {
    fn execute(content: String) -> Vec<String> {
        self::EmailScraper::run(&scrapers::email_scraper::EmailScraper { }, content)
    }
}
