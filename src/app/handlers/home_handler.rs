use axum::{http::StatusCode, Json};
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
