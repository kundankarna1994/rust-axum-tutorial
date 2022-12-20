use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};

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
