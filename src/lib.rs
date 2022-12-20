use app::routes;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
mod app;

pub async fn run() {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);
    let app = routes::routes().layer(cors);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
