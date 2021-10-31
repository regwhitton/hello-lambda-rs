use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{Context, Error};
// use serde_derive::{Deserialize, Serialize};

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Request {
//     name: String,
// }

// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// struct Response {
//     msg: String,
// }

pub(crate) async fn hello_handler(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let default_name = "World".to_string();
    let name = event.path_parameters.get("name").unwrap_or(&default_name);
    log::info!("Incoming event with name '{}'", name);

    let body = Body::Text(format!("Hello '{}'", name));

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(body),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
