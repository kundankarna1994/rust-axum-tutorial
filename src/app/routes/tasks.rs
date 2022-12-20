use crate::app::handlers::tasks_handler;
use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/create", post(tasks_handler::create))
}
