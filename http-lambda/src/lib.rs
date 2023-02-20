use lambda_http::{Body, Error, Request, Response};
use regex::Regex;
use serde::{Deserialize, Serialize};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    critic_review_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_review_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    critic_review_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_review_score: Option<String>,
}

struct IdName {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LambdaRequest {
    url: String,
}

const URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8";

pub async fn get_comic_issue_json_response(request: Request) -> Result<Response<Body>, Error> {
    let url = parse_url_from_request(&request);
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
    let critic_review_count = parse_review_count(&response, "Critic Reviews").unwrap();
    let user_review_count = parse_review_count(&response, "User Reviews").unwrap();
    let critic_review_score = parse_review_score(&response, "Critic Rating").unwrap();
    let user_review_score = parse_review_score(&response, "User Rating").unwrap();
    let comic_info = ComicInfo {
        id,
        name,
        writers,
        artists,
        publisher,
        release_date,
        cover_price,
        critic_review_count,
        user_review_count,
        critic_review_score,
        user_review_score,
    };
    let body = serde_json::to_string(&comic_info)?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .map_err(Box::new)?;
    Ok(resp)
}

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
        None => URL.to_string(),
    }
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

fn parse_review_count<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let re = Regex::new(r"(?x)(?P<count>\d+)").unwrap();
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".divider div.container ul.tabs li")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| {
            let val = name.text().collect::<Vec<_>>().last().unwrap().to_string();
            let caps = re.captures(&val).unwrap();
            caps["count"].to_string()
        });
    Ok(field)
}

fn parse_review_score<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let re = Regex::new(r"(?x)(?P<score>\d+\.\d|\d+)").unwrap();
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".issue div.container div.right div.right div")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| {
            let val = name.text().collect::<Vec<_>>().join("");
            let caps = re.captures(&val);
            match caps {
                Some(caps) => caps["score"].to_string(),
                None => "0".to_string(),
            }
        });
    Ok(field)
}
