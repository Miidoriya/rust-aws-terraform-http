#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use lambda_http::{Request, Body};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct LambdaRequest {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comic {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LambdaResponse {
    pub comics: Vec<Comic>,
}

pub const CORE_URL: &str = "https://comicbookroundup.com";

const MARVEL_URL: &str = "https://comicbookroundup.com/comic-books/reviews/marvel-comics/all-series"; //
const DC_URL: &str = "https://comicbookroundup.com/comic-books/reviews/dc-comics/all-series"; //
const IMAGE_URL: &str = "https://comicbookroundup.com/comic-books/reviews/image-comics/all-series"; //
const IDW_URL: &str = "https://comicbookroundup.com/comic-books/reviews/idw-publishing/all-series"; //
const DARK_HORSE_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/dark-horse-comics/all-series"; //
const BOOM_URL: &str = "https://comicbookroundup.com/comic-books/reviews/boom-studios/all-series"; //
const DYNAMITE_URL: &str = "https://comicbookroundup.com/comic-books/reviews/dynamite-entertainment/all-series"; //
const VALIANT_URL: &str = "https://comicbookroundup.com/comic-books/reviews/valiant-comics/all-series"; //
const VERTIGO_URL: &str = "https://comicbookroundup.com/comic-books/reviews/vertigo/all-series"; // 
const ONI_URL: &str = "https://comicbookroundup.com/comic-books/reviews/oni-press/all-series"; //
const AFTERSHOCK_URL: &str = "https://comicbookroundup.com/comic-books/reviews/aftershock-comics/all-series"; //
const ARCHIE_URL: &str = "https://comicbookroundup.com/comic-books/reviews/archie-comics/all-series"; //
const TITAN_URL: &str = "https://comicbookroundup.com/comic-books/reviews/titan-books/all-series"; //
const ZENESCOPE_URL: &str = "https://comicbookroundup.com/comic-books/reviews/zenescope-entertainment/all-series"; //
const BLACK_MASK_URL: &str = "https://comicbookroundup.com/comic-books/reviews/black-mask-studios/all-series"; //
const RED_5_URL: &str = "https://comicbookroundup.com/comic-books/reviews/red-5-comics/all-series"; //
const VAULT_URL: &str = "https://comicbookroundup.com/comic-books/reviews/vault-comics/all-series"; //

// Create a map of publisher names to their respective urls
lazy_static! {
    static ref PUBLISHER_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("marvel", MARVEL_URL);
        m.insert("dc", DC_URL);
        m.insert("image", IMAGE_URL);
        m.insert("idw", IDW_URL);
        m.insert("dark horse", DARK_HORSE_URL);
        m.insert("boom", BOOM_URL);
        m.insert("dynamite", DYNAMITE_URL);
        m.insert("valiant", VALIANT_URL);
        m.insert("vertigo", VERTIGO_URL);
        m.insert("oni", ONI_URL);
        m.insert("aftershock", AFTERSHOCK_URL);
        m.insert("archie", ARCHIE_URL);
        m.insert("titan", TITAN_URL);
        m.insert("zenescope", ZENESCOPE_URL);
        m.insert("black mask", BLACK_MASK_URL);
        m.insert("red 5", RED_5_URL);
        m.insert("vault", VAULT_URL);
        m
    };
}




pub fn parse_name_from_request(request: &Request) -> String {
    let body_str = match request.body() {
        Body::Text(s) => Some(s),
        _ => None,
    };
    match body_str {
        Some(s) => {
            let lambda_request: LambdaRequest = serde_json::from_str(s).unwrap();
            lambda_request.name
        }
        None => "".to_string(),
    }
}

pub async fn parse_comic_urls(name: &str) -> Result<LambdaResponse, reqwest::Error> {
    let url = PUBLISHER_URLS.get(&name).unwrap();
    let response_str = reqwest::get(*url).await?.text().await?;
    let html_resp = scraper::Html::parse_document(&response_str);
    let id_selector =
        scraper::Selector::parse("div.section > table > tbody > tr > td.series > a").unwrap();
    let urls: LambdaResponse = LambdaResponse { comics: html_resp
        .select(&id_selector)
        .map(|e| {
            let href = e.value().attr("href");
            let href = href.map(str::to_owned);
            let text = e.text().collect::<Vec<_>>()[0].to_owned();
            Comic {
                name: text,
                url: format!("{}{}", CORE_URL, href.unwrap()),
            }
        })
        .collect::<Vec<_>>() };
    Ok(urls)
}