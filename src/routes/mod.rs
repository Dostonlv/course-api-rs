use std::sync::Arc;

use crate::AppState;
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::Serialize;

pub mod courses;

pub async fn create_app(pool: Arc<AppState>) -> anyhow::Result<Router> {
    let router = Router::new()
        .nest("/courses", courses::router())
        .with_state(pool);

    Ok(router)
}

#[derive(Serialize)]
pub struct Data {
    pub message: String,
}

pub struct AppError {
    pub status_code: StatusCode,
    pub data: Json<Data>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.data).into_response()
    }
}
