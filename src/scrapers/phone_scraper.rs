extern crate regex;
use scrapers;
use scrapers::ScraperType;
use scrapers::regex_scraper;
use scrapers::regex_scraper::Capture;

pub struct PhoneScraper { }

impl regex_scraper::Capture for PhoneScraper {
    fn expression(&self) -> regex::Regex {
        let regex = regex::Regex::new(r"/^(?:(?:\(?(?:00|\+)([1-4]\d\d|[1-9]\d?)\)?)?[\-\.\ \\\/]?)?((?:\(?\d{1,}\)?[\-\.\ \\\/]?){0,})(?:[\-\.\ \\\/]?(?:#|ext\.?|extension|x)[\-\.\ \\\/]?(\d+))?$/i");
        regex.unwrap()
    }
}

impl scrapers::Execute for ScraperType {
    fn execute<'a>(&'a self, content: &'a String) -> Vec<String> {
        let c: String = content.clone();
        self::PhoneScraper::run(&scrapers::phone_scraper::PhoneScraper { }, c)
    }
}
