use lambda_http::{Request, Body};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct LambdaRequest {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LambdaResponse {
    pub urls: Vec<String>,
}

pub const CORE_URL: &str = "https://comicbookroundup.com";

pub fn parse_url_from_request(request: &Request) -> String {
    let body_str = match request.body() {
        Body::Text(s) => Some(s),
        _ => None,
    };
    match body_str {
        Some(s) => {
            let lambda_request: LambdaRequest = serde_json::from_str(s).unwrap();
            lambda_request.url
        }
        None => "".to_string(),
    }
}

pub async fn parse_comic_issue_urls(url: &str) -> Result<Vec<Option<String>>, reqwest::Error> {
    let response_str = reqwest::get(url).await?.text().await?;
    let html_resp = scraper::Html::parse_document(&response_str);
    let id_selector =
        scraper::Selector::parse("div.section > table > tbody > tr > td.issue > a").unwrap();
    let urls = html_resp
        .select(&id_selector)
        .map(|e| e.value().attr("href").map(str::to_owned))
        .collect();
    Ok(urls)
}