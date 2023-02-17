use std::sync::Arc;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ComicInfo {
    name: String,
    writer: String,
    artist: String,
    publisher: String,
    release_date: String,
    cover_price: String,
}

const URL: &str = "https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (_event, _context) = event.into_parts();
    let response = get_response(URL).await?;
    let name = parse_name(&response).unwrap();
    let writer = parse_comic_info_field(&response, "Writer").unwrap();
    let artist = parse_comic_info_field(&response, "Artist").unwrap();
    let publisher = parse_comic_info_field(&response, "Publisher").unwrap();
    let release_date = parse_comic_info_field(&response, "Release Date").unwrap();
    let cover_price = parse_comic_info_field(&response, "Cover Price").unwrap();
    let comic_info = ComicInfo {
        name,
        writer,
        artist,
        publisher,
        release_date,
        cover_price,
    };
    Ok(json!({
        "statusCode": 200,
        "body": comic_info,
    }))
}

async fn get_response(url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url)
    .await?
    .text()
    .await?;
    Ok(response)
}

fn parse_name(response: &str) -> Result<String, scraper::error::SelectorErrorKind> {
    let response = scraper::Html::parse_document(response);
    let name_selector =
        scraper::Selector::parse("div.series-buttons a.series")?;
    let name = match response.select(&name_selector).next() {
        Some(name) => name.value().attr("href").unwrap().to_string().split('/').last().unwrap().to_string(),
        None => "N/A".to_string(),
    };
    Ok(name)
}

fn parse_comic_info_field<'a>(response: &'a str, field: &'a str) -> Result<String, scraper::error::SelectorErrorKind<'a>> {
    let response = scraper::Html::parse_document(response);
    let name_selector =
        scraper::Selector::parse(".issue div.container div.right div.left span")?;
    let field = match response.select(&name_selector).find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field)) {
        Some(name) => name.text().collect::<Vec<_>>().last().unwrap().to_string(),
        None => "N/A".to_string(),
    };
    Ok(field)
}

