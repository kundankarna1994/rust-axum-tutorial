mod home;

use axum::Router;

pub fn routes() -> Router {
    Router::new().nest("/", home::routes())
}
