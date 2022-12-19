use crate::app::handlers::home_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home_handler::index))
        .route("/mirror_body_string", get(home_handler::mirror_body_string))
        .route("/mirror_body_json", get(home_handler::mirror_body_json))
        .route("/path_variables/:id", get(home_handler::path_variables))
        .route(
            "/path_variables/15", // ordering doesnot matter in axum, it still works
            get(home_handler::path_variables_hard_coded),
        )
}
