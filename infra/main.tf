module "basic_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "basic-lambda-function"
  description   = "A basic lambda function"
  handler       = "bootstrap"
  runtime       = "provided.al2"

  create_package         = false
  local_existing_package = "../release/index.zip"

  timeout = 60

  tags = {
    Name = "basic-lambda-function"
  }
}