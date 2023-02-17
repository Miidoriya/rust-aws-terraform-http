use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

const URL: &str = "https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let response = get_response(URL).await?;
    let name = parse_name(&response);
    let name = match name {
        Ok(name) => name,
        Err(_) => "Some error retrieving name".to_string(),
    };
    Ok(json!({
        "statusCode": 200,
        "body": name
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
    let response = scraper::Html::parse_document(&response);
    let name_selector =
        scraper::Selector::parse("div.series-buttons a.series")?;
    let name = match response.select(&name_selector).next() {
        Some(name) => name.value().attr("href").unwrap().to_string(),
        None => "N/A".to_string(),
    };
    Ok(name)
}

