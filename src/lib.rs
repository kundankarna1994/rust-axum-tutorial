use router::routes;

mod handlers;
mod router;

pub async fn run() {
    let app = routes();
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
