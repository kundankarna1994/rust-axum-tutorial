use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

pub async fn validate_with_serde(Json(req_user): Json<RequestUser>) {
    println!("{:?}", req_user.username);
    println!("{:?}", req_user.password);
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(user)
    }
}
pub async fn custom_json_extractor(user: RequestUser) {
    println!("{:?}", user);
}
