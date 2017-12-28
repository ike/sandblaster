extern crate regex;
use self::regex::Regex;

pub fn run(content: String) -> Vec<String> {
    let expr = Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").unwrap();
    let captures = expr.find_iter(&content);
    let mut emails = Vec::new();

    for cap in captures {
        emails.push(String::from(cap.as_str()));
    }

    emails.dedup();

    emails
}
