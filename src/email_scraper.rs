extern crate regex;
use regex_scraper;

pub struct EmailScraper { }

impl regex_scraper::Capture for EmailScraper {
    fn expression(&self) -> regex::Regex {
        let regex = regex::Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?");
        regex.unwrap()
    }
}
