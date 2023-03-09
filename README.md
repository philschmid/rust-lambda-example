# AWS Lambda Example for Rust

This is repository includes an example of how to use Rust on AWS Lambda with an API Gateway. The repository supports local development and deployment to AWS Lambda. 

#### Supported Features

- [x] Local development with `cargo lambda watch`
- [x] parsing of API Gateway events 
- [x] deployment to AWS Lambda with `cdk deploy`

## Local Development

To run the example locally, you need to install the `cargo-lambda` tool. You can install it with `cargo install cargo-lambda`.

After that, you can run the example with `cargo lambda watch`. This will start a local server on port `9000` that will listen for API Gateway events. The url for the local server is on `http://localhost:9000/lambda-url/{CARGO TOML PACKAGE NAME}` for us `http://localhost:9000/lambda-url/sample-api`.

```bash
curl --request POST \
  --url http://localhost:9000/lambda-url/sample-api \
  --header 'Content-Type: application/json' \
  --data '{"id":"bar","name":"test","age":1}'
```

The best part about `cargo-lambda watch` is that it will automatically recompile your code when you make changes. This means that you can make changes to your code and immediately test them without having to restart the server.

## Deployment

To deploy the example to AWS Lambda, you need to install the AWS CDK. You can install it with `npm install -g aws-cdk`.

After that, you can deploy the example with `cdk deploy`. This will deploy the example to AWS Lambda and create an API Gateway endpoint for you. The url for the API Gateway endpoint is on the output of the `cdk deploy` command. The Rust function will created with [Cargo Lambda CDK construct](https://github.com/cargo-lambda/cargo-lambda-cdk)

```bash
cd infrastructure
cdk deploy
```


