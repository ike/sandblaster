extern crate reqwest;
use self::reqwest::Client;
use self::reqwest::StatusCode;
use email_scraper;

#[derive(Debug)]
pub enum GetError {
    ReqwestError
}

impl From<reqwest::Error> for GetError {
    fn from(_e: reqwest::Error) -> GetError {
        GetError::ReqwestError
    }
}

pub fn get(domain: &str) -> Result<Vec<String>, GetError> {
    let client = Client::new();

    let mut resp = client.get(domain).send()?;
    let content = resp.text();
    let mut emails = Vec::new();

    match resp.status() {
        StatusCode::Ok => {
            match content {
                Ok(content) => emails = email_scraper::run(content),
                Err(error) => panic!("{:?}", error)
            }
        }
        status => panic!("{:?}", status)
    };

    Ok(emails)
}
