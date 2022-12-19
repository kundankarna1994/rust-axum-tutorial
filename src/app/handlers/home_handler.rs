use axum::Json;
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
pub async fn mirror_body_json(Json(payload): Json<Payload>) -> Json<Payload> {
    println!("{:?}", payload);
    Json(payload)
}
