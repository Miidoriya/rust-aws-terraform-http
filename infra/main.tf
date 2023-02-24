module "issue_urls_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "issue-urls-lambda"
  description   = "A function to act as an API for retrieving a JSON struct of urls representing a comics available issues"
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"]

  create_package         = false
  local_existing_package = "../release/issue-urls.zip"

  timeout = 60

  tags = {
    Name = "issue-urls-lambda"
  }
}

module "issue_details_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "issue-details-lambda"
  description   = "A function to act as an API for retrieving a JSON struct of comic issue details"
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"]

  create_package         = false
  local_existing_package = "../release/issue-details.zip"

  timeout = 60

  tags = {
    Name = "issue-details-lambda"
  }
}

module "publisher_comics_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "publisher-comics-lambda"
  description   = "A function to act as an API for retrieving a JSON struct of comics for a given publisher"
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"]

  create_package         = false
  local_existing_package = "../release/publisher-comics.zip"

  timeout = 60

  tags = {
    Name = "publisher-comics-lambda"
  }
}

resource "aws_lambda_function_url" "issue_urls" {
  function_name      = module.issue_urls_lambda_function.lambda_function_name
  authorization_type = "NONE"
}

resource "aws_lambda_function_url" "issue_details" {
  function_name      = module.issue_details_lambda_function.lambda_function_name
  authorization_type = "NONE"
}

resource "aws_lambda_function_url" "publisher_comics" {
  function_name      = module.publisher_comics_lambda_function.lambda_function_name
  authorization_type = "NONE"
}


output "issue_urls_function_url" {
  value = aws_lambda_function_url.issue_urls.function_url
}

output "issue_details_function_url" {
  value = aws_lambda_function_url.issue_details.function_url
}

output "publisher_comics_function_url" {
  value = aws_lambda_function_url.publisher_comics.function_url
}