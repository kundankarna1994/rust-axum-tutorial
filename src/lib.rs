use app::routes;
mod app;

pub async fn run() {
    let app = routes::routes();
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
