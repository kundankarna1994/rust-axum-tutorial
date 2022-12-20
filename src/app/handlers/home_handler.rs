use axum::{
    extract::{Path, Query},
    headers::UserAgent,
    http::{HeaderMap, StatusCode},
    Extension, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::app::middlewares::shared_data_middleware::SharedData;

pub async fn index() -> String {
    "Hello From the Server".to_owned()
}

pub async fn mirror_body_string(body: String) -> String {
    body
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorPayloadResponse {
    message: String,
    custom_message: String,
}
pub async fn mirror_body_json(Json(payload): Json<Payload>) -> Json<MirrorPayloadResponse> {
    println!("{:?}", payload);
    Json(MirrorPayloadResponse {
        message: payload.message,
        custom_message: "This is custom message from server".to_owned(),
    })
}

pub async fn path_variables(Path(id): Path<u32>) -> String {
    id.to_string()
}

pub async fn path_variables_hard_coded() -> String {
    "Fifteen".to_owned()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
}

pub async fn query_params(Query(params): Query<QueryParams>) -> Json<QueryParams> {
    println!("{:?}", params);
    Json(params)
}

pub async fn header_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

pub async fn custom_header(headers: HeaderMap) -> String {
    let message = headers.get("x-custom_header").unwrap();
    let message = message.to_str().unwrap().to_owned();
    message
}

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    return shared_data.message;
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);
pub async fn custom_middleware(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn always_error() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
