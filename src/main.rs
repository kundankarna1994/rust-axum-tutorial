use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    //create a new router

    let app = Router::new().route("/hello", get(hello_world));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String {
    "Hello World!!!!!".to_owned()
}
