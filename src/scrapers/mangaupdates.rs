extern crate reqwest;
extern crate scraper;

pub struct MUScraper {
  daily_releases_url: String,
}
const release_title_selector_raw: String =
  "#main_content > div > div:nth-child(4) > table > tbody > tr > td.pad > a";
const release_chapter_selector_raw: String =
  "#main_content > div > div:nth-child(4) > table > tbody > tr > td:nth-child(2)";

const release_group_selector_raw: String =
  "#main_content > div > div:nth-child(4) > table > tbody > tr > td:nth-child(3) > a";

pub struct MUDailyReleaseInfo {
  name: String,
  url: String,
  latest_release: String,
  translator: String,
  translator_url: String,
}

impl MUScraper {
  fn scrapeDailyReleasePage(&self) -> Result<Vec<MUDailyReleaseInfo>, String> {
    let response_html = reqwest::get(self.mu_releases_url)?.text()?;
    let parsed_html = scraper::Html::parse_document(mu_response_html.as_str());

    let release_title_selector = scraper::Selector::parse(release_title_selector_raw);
    let release_chapter_selector = scraper::Selector::parse(release_chapter_selector_raw);
    let release_group_selector = scraper::Selector::parse(release_group_selector_raw);

    let dailyReleases: Vec<MUDailyReleaseInfo>;

    for title in parsed_html.select(&mu_release_title_selector) {
      dailyReleases.push(MUDailyReleaseInfo {
        name: title.inner_html(),
        url: title.value().attr("href")?,
      });
      println!("manga names = {}", title.inner_html());
      println!("manga url = {}", title.value().attr("href").unwrap());
    }

    for ch in parsed_html.select(&mu_release_chapter_selector) {
      println!("manga chapter = {}", ch.inner_html());
    }

    for group in parsed_html.select(&mu_release_group_selector) {
      println!("manga group = {}", group.inner_html());
      println!("manga group url = {}", group.value().attr("href").unwrap());
    }
  }
}
