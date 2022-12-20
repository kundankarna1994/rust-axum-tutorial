use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn validate_with_serde(Json(req_user): Json<RequestUser>) {
    println!("{:?}", req_user.username);
    println!("{:?}", req_user.password);
}
