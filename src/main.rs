extern crate reqwest;
// extern crate scraper;
use std::env;
use std::process;

const HELP_STR: &str = "
    manga-memorial api|scrape

    api - Expose REST API for providing RSS feed and updating feed
    scrape - Poll data from various sources periodically
";

fn scrape() {
    let mu_releases_url = "https://www.mangaupdates.com/releases.html";
    let html = reqwest::get(mu_releases_url).unwrap().text().unwrap();
    println!("Successfully got \n{}", html);
    // #main_content > div > div:nth-child(4) > table > tbody > tr:nth-child(6) > td.pad > a
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", HELP_STR);
        process::exit(0);
    }

    match args[1].as_str() {
        "scrape" => scrape(),
        _ => {
            println!("{}", HELP_STR);
            process::exit(0);
        }
    }
}
