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

pub fn get(domains: Vec<&str>, scrapers: Vec<&str>) -> Result<Vec<String>, GetError> {
    let client = Client::new();
    let mut emails = Vec::new();

    for domain in domains {
        let mut resp = client.get(domain).send()?;
        let content = resp.text();

        match resp.status() {
            StatusCode::Ok => {
                match content {
                    Ok(content) => emails.extend(email_scraper::run(content).iter().cloned()),
                    Err(error) => panic!("{:?}", error)
                }
            }
            status => panic!("{:?}", status)
        };
    }

    Ok(emails)
}
