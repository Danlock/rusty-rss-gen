mod mscrapers;

use std::env;
use std::process;

const HELP_INFO: &str = "
    manga-memorial api|scrape

    api - Expose REST API for providing RSS feed and updating feed
    scrape - Poll data from various sources periodically
";

fn scrape_and_save_to_db() {
    let manga_updates_scraper = mscrapers::MUScraper {
        daily_releases_url: "https://www.mangaupdates.com/releases.html",
    };
    for m in manga_updates_scraper.scrape_daily_release_page().unwrap() {
        println!("Manga : {:?}", m);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", HELP_INFO);
        process::exit(0);
    }

    match args[1].as_str() {
        "scrape" => scrape_and_save_to_db(),
        "api" => {
            println!("Unimplemented!");
            process::exit(0);
        }
        _ => {
            println!("{}", HELP_INFO);
            process::exit(0);
        }
    }
}
