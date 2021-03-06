extern crate regex;
use scrapers;
use scrapers::ScraperType;
use scrapers::regex_scraper;
use scrapers::regex_scraper::Capture;

pub struct EmailScraper { }

impl regex_scraper::Capture for EmailScraper {
    fn expression(&self) -> regex::Regex {
        let regex = regex::Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?");
        regex.unwrap()
    }
}

impl scrapers::Execute for ScraperType {
    fn execute<'a>(&'a self, content: &'a String) -> Vec<String> {
        let c: String = content.clone();
        self::EmailScraper::run(&scrapers::email_scraper::EmailScraper { }, c)
    }
}
