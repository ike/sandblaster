extern crate reqwest;
use self::reqwest::Client;
use self::reqwest::StatusCode;

use scrapers;

#[derive(Debug)]
pub enum GetError {
    ReqwestError
}

impl From<reqwest::Error> for GetError {
    fn from(_e: reqwest::Error) -> GetError {
        GetError::ReqwestError
    }
}

pub fn get<'a>(domains: Vec<&str>, scrapers: Vec<&'a str>) -> Result<Vec<String>, GetError> {
    let client = Client::new();
    let mut results = Vec::new();
    let mut content: String;

    for domain in domains {
        let mut resp = client.get(domain).send()?;

        match resp.status() {
            StatusCode::OK => {
                match resp.text() {
                    Ok(text) => content = text,
                    Err(error) => panic!("{:?}", error)
                }
            }
            status => panic!("{:?}", status)
        };

        for scraper in &scrapers {
            let scraper_enum = scraper.parse::<scrapers::Scrapers>();
            match scraper_enum {
                Ok(scraper_enum) => results.extend(scraper_enum.execute(&content)),
                Err(error) => panic!("{:?}", error)
            }
        }
    }

    Ok(results)
}
