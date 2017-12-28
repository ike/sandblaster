#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod domain;
mod email_scraper;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let domain = matches.value_of("domains").unwrap();

    let results = domain::get(domain);

    match results {
        Ok(results) => println!("{:?}", results),
        Err(error) => panic!("{:?}", error)
    }
}
