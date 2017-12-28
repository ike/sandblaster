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
    let mut results = Vec::new();
    let mut content = String::new();

    for domain in domains {
        let mut resp = client.get(domain).send()?;

        match resp.status() {
            StatusCode::Ok => {
                match resp.text() {
                    Ok(text) => content = text,
                    Err(error) => panic!("{:?}", error)
                }
            }
            status => panic!("{:?}", status)
        };
        results.extend(email_scraper::run(content));
    }

    Ok(results)
}
