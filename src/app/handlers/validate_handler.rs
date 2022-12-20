use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn validate_with_serde(Json(req_user): Json<RequestUser>) {
    println!("{:?}", req_user.username);
    println!("{:?}", req_user.password);
}

#[async_trait]
impl<B> FromRequest<B> for RequestUser
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req.extract::<Json<RequestUser>>().await;
    }
}
pub async fn custom_json_extractor(Json(user): Json<RequestUser>) {
    println!("{:?}", user);
}
