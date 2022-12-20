use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::app::database::tasks;

#[derive(Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub priority: String,
}
pub async fn create(Extension(db): Extension<DatabaseConnection>, Json(task): Json<Task>) {
    let entity = tasks::ActiveModel {
        title: Set(task.title),
        priority: Set(task.priority),
        ..Default::default()
    };

    let result = entity.save(&db).await.unwrap();
    dbg!(result);
}
