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

enum GetError {
    ReqwestError
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

    get(domain);
}

fn get(domain: &str) -> Result<(), GetError> {
    let client = Client::new();

    let mut resp = client.get(domain).send()?;
    let content = resp.text();

    match resp.status() {
        StatusCode::Ok => {
            match content {
                Ok(content) => email_scraper(content),
                Err(e) => println!("error parsing content: {:?}", e)
            }
        }
        s => println!("Received response status: {:?}", s)
    };
    Ok(())
}

fn email_scraper(content: String) {
    // println!("{:#?}", content);
    let expr = Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").unwrap();
    let captures = expr.captures(&content);

    match captures {
        None => {
            println!("Emails found: {:?}", 0);
        }
        Some(captures) => {
            let captures_iter = captures.iter();
            for cap in captures_iter {
                if let Some(cap) = cap {
                    println!("{:?}", cap.as_str());
                }
            }
        }
    }
}
