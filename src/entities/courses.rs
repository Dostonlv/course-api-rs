use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, ToSchema)]
pub struct Course {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub instructor_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourse {
    pub title: String,
    pub description: String,
    pub instructor_id: i64,
}

pub type UpdateCourse = CreateCourse;
