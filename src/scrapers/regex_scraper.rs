extern crate regex;

pub trait Capture {
    fn run(&self, content: String) -> Vec<String> {
        let expression = self.expression();
        let captures = expression.find_iter(&content);
        let mut results = Vec::new();

        for cap in captures {
            results.push(String::from(cap.as_str()));
        }

        results.dedup();

        results
    }

    fn expression(&self) -> regex::Regex;
}
