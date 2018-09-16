extern crate reqwest;
extern crate scraper;

use std::env;
use std::process;

const HELP_STR: &str = "
    manga-memorial api|scrape

    api - Expose REST API for providing RSS feed and updating feed
    scrape - Poll data from various sources periodically
";

fn scrape() {
    let mu_releases_url = "https://www.mangaupdates.com/releases.html";
    let mu_response_html = reqwest::get(mu_releases_url).unwrap().text().unwrap();

    let mu_release_html_parsed = scraper::Html::parse_document(mu_response_html.as_str());

    let mu_release_title_selector = scraper::Selector::parse(
        "#main_content > div > div:nth-child(4) > table > tbody > tr > td.pad > a",
    ).unwrap();
    let mu_release_chapter_selector = scraper::Selector::parse(
        "#main_content > div > div:nth-child(4) > table > tbody > tr > td:nth-child(2)",
    ).unwrap();
    let mu_release_group_selector = scraper::Selector::parse(
        "#main_content > div > div:nth-child(4) > table > tbody > tr > td:nth-child(3) > a",
    ).unwrap();

    for title in mu_release_html_parsed.select(&mu_release_title_selector) {
        println!("manga names = {}", title.inner_html());
        println!("manga url = {}", title.value().attr("href").unwrap());
    }

    for ch in mu_release_html_parsed.select(&mu_release_chapter_selector) {
        println!("manga chapter = {}", ch.inner_html());
    }

    for group in mu_release_html_parsed.select(&mu_release_group_selector) {
        println!("manga group = {}", group.inner_html());
        println!("manga group url = {}", group.value().attr("href").unwrap());
    }
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
