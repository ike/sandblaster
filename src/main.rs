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
    fn from(e: reqwest::Error) -> GetError {
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
    println!("Emails found: \n");

    let expr = Regex::new(r"\A[^@]+@([^@\.]+\.)+[^@\.]+\z").unwrap();
    let captures = expr.captures(&content);

    for cap in captures {
        println!("{:#?}, \n", cap);
    }
}
