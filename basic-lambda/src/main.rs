use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct ComicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<String>,
     #[serde(skip_serializing_if = "Option::is_none")]
    cover_price: Option<String>,
}

struct IdName {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LambdaRequest {
    url: String,
}

const URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    println!("body.url: {}", event["body"]["url"]);
    println!("body: {}", event["body"]);
    println!("event: {}", event);
    let body: Option<LambdaRequest> = event["body"].as_str().map(|body| serde_json::from_str(body).unwrap());
    let url: String = match body {
        Some(body) => {
            let body: LambdaRequest = body;
            body.url
        }
        None => {
            URL.to_string()
        }
    };
    let response = get_response(&url).await?;
    let id_name = parse_name(&response).unwrap();
    let id = id_name.id;
    let name = id_name.name;
    let writers: Option<Vec<String>> =
        parse_comic_info_field(&response, "Writer")
            .unwrap()
            .map(|writers| {
                writers
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            });
    let artists = parse_comic_info_field(&response, "Artist")
        .unwrap()
        .map(|artists| {
            artists
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });
    let publisher = parse_comic_info_field(&response, "Publisher").unwrap();
    let release_date = parse_comic_info_field(&response, "Release Date").unwrap();
    let cover_price = parse_comic_info_field(&response, "Cover Price").unwrap();
    let comic_info = ComicInfo {
        id,
        name,
        writers,
        artists,
        publisher,
        release_date,
        cover_price,
    };
    Ok(json!({
        "statusCode": 200,
        "headers": {
            "content-type": "application/json"
        },
        "body": comic_info,
    }))
}

async fn get_response(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

fn parse_name(response: &str) -> Result<IdName, scraper::error::SelectorErrorKind> {
    let response = scraper::Html::parse_document(response);
    let id_selector = scraper::Selector::parse("div.series-buttons a.series")?;
    let name_selector = scraper::Selector::parse(".issue div.container div.right h1 span")?;
    let name = response
        .select(&name_selector)
        .next()
        .map(|name| name.text().collect::<Vec<_>>().join(""));
    let id = response.select(&id_selector).next().map(|name| {
        name.value()
            .attr("href")
            .unwrap()
            .to_string()
            .split('/')
            .last()
            .unwrap()
            .to_string()
    });
    let id_name = IdName { id, name };
    Ok(id_name)
}

fn parse_comic_info_field<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".issue div.container div.right div.left span")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| name.text().collect::<Vec<_>>().last().unwrap().to_string());
    Ok(field)
}

//{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/11"}\

// curl -X POST \
//     'https://dfz3qumzmo4ryzxhvb72q5ryl40eaaea.lambda-url.eu-west-2.on.aws/' \
//     -H 'Content-Type: application/json' \
//     -d '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/11"}'
