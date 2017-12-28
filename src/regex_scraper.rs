extern crate regex;
use self::regex::Regex;

pub struct RegexScraper {
    expression: regex::Regex
}

pub trait Capture {
    fn run(&self, content: String) -> Vec<String> {
        let captures = self.expression.find_iter(&content);
        let mut results = Vec::new();

        for cap in captures {
            results.push(String::from(cap.as_str()));
        }

        results.dedup();

        results
    }
}
