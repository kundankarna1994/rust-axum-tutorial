use axum::Router;
mod home;
pub fn routes() -> Router {
    Router::new().nest("/", home::routes())
}
