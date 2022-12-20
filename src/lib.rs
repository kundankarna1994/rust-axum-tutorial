use std::env;

use app::{middlewares::custom_middleware::set_custom_header_middleware, routes};
use axum::{http::Method, middleware, Extension};
use sea_orm::Database;
use tower_http::cors::{Any, CorsLayer};
mod app;
use crate::app::middlewares::shared_data_middleware;

pub async fn run() {
    let db_uri = env::var("DATABASE_URL").expect("DB URI Not Found");
    let _db = Database::connect(db_uri).await.unwrap(); //Todo::
    let shared_data = shared_data_middleware::shared_data();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let app = routes::routes()
        .layer(cors)
        .layer(Extension(shared_data))
        .route_layer(middleware::from_fn(set_custom_header_middleware));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
