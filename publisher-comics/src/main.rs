use lambda_http::{run, service_fn, Body, Error, Request, Response};
use publisher_comics::{parse_name_from_request, parse_comic_urls};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let name = parse_name_from_request(&event);
    let urls = parse_comic_urls(&name).await.unwrap();
    let resp_json = serde_json::to_string(&urls).unwrap();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from(resp_json))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

// curl --location 'http://localhost:9000/lambda-url/publisher-comics/' \
// --header 'Content-Type: application/json' \
// --data '{"name":"marvel"}' | json_pp