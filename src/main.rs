#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate reqwest;
extern crate regex;
use reqwest::Client;
use reqwest::StatusCode;
use regex::Regex;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Debug)]
enum GetError {
    ReqwestError,
    ContentError,
    StatusError
}

#[derive(Debug)]
enum ScraperError {
    ScrapeError
}

impl From<reqwest::Error> for GetError {
    fn from(_e: reqwest::Error) -> GetError {
        GetError::ReqwestError
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let domain = matches.value_of("domains").unwrap_or("http://ike.io");

    let results = get(domain);

    match results {
        Ok(results) => println!("{:?}", results),
        Err(error) => panic!("{:?}", error)
    }
}

fn get(domain: &str) -> Result<Vec<String>, GetError> {
    let client = Client::new();

    let mut resp = client.get(domain).send()?;
    let content = resp.text();
    let mut emails = Vec::new();

    match resp.status() {
        StatusCode::Ok => {
            match content {
                Ok(content) => emails = email_scraper(content).unwrap(),
                Err(error) => panic!("{:?}", error)
            }
        }
        status => panic!("{:?}", status)
    };

    Ok(emails)
}

fn email_scraper(content: String) -> Result<Vec<String>, ScraperError> {
    let expr = Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").unwrap();
    let captures = expr.find_iter(&content);
    let mut emails = Vec::new();

    for cap in captures {
        emails.push(String::from(cap.as_str()));
    }

    emails.dedup();

    Ok(emails)
}
