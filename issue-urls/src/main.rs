use lambda_http::{run, service_fn, Body, Error, Request, Response};
use issue_urls::{parse_url_from_request, parse_comic_issue_urls, LambdaResponse};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let url = parse_url_from_request(&event);
    let urls = parse_comic_issue_urls(&url).await.unwrap();
    let lambda_response = LambdaResponse {
        urls: urls
            .iter()
            .filter_map(|url| url.as_ref().map(|s| format!("{}{}", issue_urls::CORE_URL, s)))
            .collect(),
    }; 
    let resp_json = serde_json::to_string(&lambda_response).unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from(resp_json))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
