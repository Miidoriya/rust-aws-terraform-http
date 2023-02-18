module "basic_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "basic-lambda-function"
  description   = "A basic lambda function"
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"]

  create_package         = false
  local_existing_package = "../release/index.zip"

  timeout = 60

  tags = {
    Name = "basic-lambda-function"
  }
}

resource "aws_lambda_function_url" "test_latest" {
  function_name      = module.basic_lambda_function.lambda_function_name
  authorization_type = "NONE"
}

output "name" {
  value = aws_lambda_function_url.test_latest.function_url
}