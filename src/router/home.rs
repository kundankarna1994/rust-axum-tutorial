use crate::handlers::home_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(home_handler::index))
}
