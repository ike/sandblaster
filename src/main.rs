#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod domain;
mod regex_scraper;
mod email_scraper;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let domains: Vec<&str> = matches.values_of("domains").unwrap().collect();
    let scrapers: Vec<&str> = matches.values_of("scrapers").unwrap().collect();

    let results = domain::get(domains, scrapers);

    match results {
        Ok(results) => println!("{:?}", results),
        Err(error) => panic!("{:?}", error)
    }
}
