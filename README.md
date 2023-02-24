# rust-aws-terraform-http.
Requires:
- terraform
- aws credentials
- cargo
    - Install `cargo lambda`

Run ./deploy-to-tf.sh to:
- build and zip the rust binary
- terraform deploy the rust binary as a lambda to AWS
- Returns a url in the format `https://<hash>.lambda-url.<region>.on.aws/`
    - Can be queied via browser or curl command like the following example 
    ```
    curl --location 'https://<hash>.lambda-url.<region>.on.aws/' \
    --header 'Content-Type: application/json' \
    --data '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/1"}' | json_pp
    ```
    - `json_pp` pretty prints the response as JSON to the CLI

Local deployment and running:
- run `cargo lambda watch`
- curl locally or visit the endpoint via your choice of browser:

```
curl --location 'http://localhost:9000/lambda-url/issue-urls/' \
--header 'Content-Type: application/json' \
--data '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)"}' | json_pp

curl --location 'http://localhost:9000/lambda-url/issue-details/' \
--header 'Content-Type: application/json' \
--data '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8"}' | json_pp

curl --location 'http://localhost:9000/lambda-url/publisher-comics/' \
--header 'Content-Type: application/json' \
--data '{"name":"marvel"}' | json_pp
```
