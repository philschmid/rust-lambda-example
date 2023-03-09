use anyhow::{bail, Result};
use aws_lambda_events::{apigw::ApiGatewayV2httpResponse, event::apigw::ApiGatewayV2httpRequest};
use lambda_http::{service_fn, Body, Error};
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

// fn extract_request_body(request: LambdaEvent<ApiGatewayV2httpRequest>) -> Option<String> {
//     let body = match request.payload.body {
//         Some(id) => id,
//         None => {
//             tracing::error!("Body not found");

//             return Option::None;
//         }
//     };

//     tracing::info!("body: {}", body);

//     if body.len() == 0 {
//         return Option::None;
//     }

//     Some(body)
// }
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    id: String,
    name: String,
    age: u8,
}

fn parse_request_body(payload: ApiGatewayV2httpRequest) -> Result<User> {
    let body = match payload.body {
        Some(body_string) => from_str::<User>(body_string.as_str())?,
        None => {
            bail!("Body not found");
        }
    };
    tracing::info!("body: {:?}", body);
    Ok(body)
}
async fn handler(
    request: LambdaEvent<ApiGatewayV2httpRequest>,
) -> Result<ApiGatewayV2httpResponse, Error> {
    // extract body from request
    let user = match parse_request_body(request.payload) {
        Ok(user) => user,
        Err(err) => {
            tracing::error!("Error: {}", &err);
            let err = JsonError {
                message: err.to_string(),
            };
            return Ok(ApiGatewayV2httpResponse {
                status_code: 400,
                body: Some(Body::Text(to_string(&err)?)),
                ..Default::default()
            });
        }
    };

    let response = ApiGatewayV2httpResponse {
        status_code: 200,
        body: Some(Body::Text(to_string(&user)?)),
        ..Default::default()
    };
    Ok(response)
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

    // lambda_runtime::run(service_fn(
    //     |request: LambdaEvent<ApiGatewayV2httpRequest>| handler(request),
    // ))
    // .await;
    lambda_runtime::run(service_fn(handler)).await?;

    Ok(())
}
