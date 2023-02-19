use lambda_http::{run, service_fn, Body, Error, Request, Response};
use http_lambda::{get_comic_issue_json_response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(func)).await
}

async fn func(request: Request) -> Result<Response<Body>, Error> {
    get_comic_issue_json_response(request).await
}

