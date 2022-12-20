use dotenvy::dotenv;
use rust_axum::run;
#[tokio::main]
async fn main() {
    dotenv().ok();
    // run the axum server
    run().await;
}
