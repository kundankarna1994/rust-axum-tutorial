use crate::app::handlers::home_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home_handler::index))
        .route("/mirror_body_string", get(home_handler::mirror_body_string))
}