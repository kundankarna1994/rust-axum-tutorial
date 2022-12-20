use axum::Router;

mod home;
mod tasks;

pub fn routes() -> Router {
    Router::new()
        .nest("/", home::routes())
        .nest("/tasks", tasks::routes())
}
