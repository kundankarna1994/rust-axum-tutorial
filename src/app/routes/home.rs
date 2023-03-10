use crate::app::handlers::{home_handler, validate_handler};
use axum::{
    routing::{get, post},
    Router,
};

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
        .route("/query_params", get(home_handler::query_params))
        .route("/header_user_agent", get(home_handler::header_user_agent))
        .route("/custom_header", get(home_handler::custom_header))
        .route("/middleware_message", get(home_handler::middleware_message))
        .route("/custom_middleware", get(home_handler::custom_middleware))
        .route("/always_error", get(home_handler::always_error))
        .route("/created", get(home_handler::created))
        .route("/send_json_data", get(home_handler::send_json_data))
        .route(
            "/validate_with_serde",
            get(validate_handler::validate_with_serde),
        )
        .route(
            "/custom_json_extractor",
            post(validate_handler::custom_json_extractor),
        )
}
