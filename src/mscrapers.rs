extern crate failure;
extern crate reqwest;
extern crate scraper;

#[derive(Debug)]
pub struct MUDailyReleaseInfo {
  name: String,
  url: String,
  latest_release: String,
  translator: String,
  translator_url: String,
}

pub struct MUScraper {
  pub daily_releases_url: &'static str,
}
const RELEASE_ROW_SELECTOR_RAW: &'static str =
  "#main_content > div > div:nth-child(4) > table > tbody > tr";

const RELEASE_TITLE_SELECTOR_RAW: &'static str = "td.pad > a";
const RELEASE_CHAPTER_SELECTOR_RAW: &'static str = "td:nth-child(2)";
const RELEASE_GROUP_SELECTOR_RAW: &'static str = "td:nth-child(3) > a";

impl From<ParseError> for failure::Error {
    fn from(e: ParseError) -> Self {
        failure::err_msg("Oops")
    }
}

impl MUScraper {
  pub fn scrape_daily_release_page(&self) -> Result<Vec<MUDailyReleaseInfo>, failure::Error> {
    let response_html = reqwest::get(self.daily_releases_url)?.text()?;
    let parsed_html = scraper::Html::parse_document(response_html.as_str());

    let release_selector = scraper::Selector::parse(RELEASE_ROW_SELECTOR_RAW)?;
    let release_title_selector = scraper::Selector::parse(RELEASE_TITLE_SELECTOR_RAW).unwrap();
    let release_chapter_selector = scraper::Selector::parse(RELEASE_CHAPTER_SELECTOR_RAW).unwrap();
    let release_group_selector = scraper::Selector::parse(RELEASE_GROUP_SELECTOR_RAW).unwrap();

    let mut daily_releases = Vec::new();

    for row in parsed_html.select(&release_selector) {
      println!("Got row: {}", row.html());
      let titles: Vec<scraper::ElementRef> = row.select(&release_title_selector).collect();
      if titles.len() != 1 {
        return Err(failure::err_msg("Didnt get the expected titles!"));
      }
      let chapters: Vec<scraper::ElementRef> = row.select(&release_chapter_selector).collect();
      if chapters.len() != 1 {
        return Err(failure::err_msg("Didnt get the expected chapters!"));
      }

      let groups: Vec<scraper::ElementRef> = row.select(&release_group_selector).collect();
      if groups.len() != 1 {
        return Err(failure::err_msg("Didnt get the expected groups!"));
      }

      let daily_info = MUDailyReleaseInfo {
        name: titles[0].inner_html(),
        url: String::from(titles[0].value().attr("href").unwrap()),
        latest_release: chapters[0].inner_html(),
        translator: groups[0].inner_html(),
        translator_url: String::from(groups[0].value().attr("href").unwrap()),
      };
      daily_releases.push(daily_info);
    }

    return Ok(daily_releases);
  }
}
