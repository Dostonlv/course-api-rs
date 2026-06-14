use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::entities::instructors::Instructor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, ToSchema, Default)]
pub struct Course {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    #[sqlx(flatten)]
    pub instructor: Instructor,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCourse {
    pub title: String,
    pub description: String,
    pub instructor_id: i64,
}

pub type UpdateCourse = CreateCourse;
