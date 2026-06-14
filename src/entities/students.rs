use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, FromRow, ToSchema)]
pub struct Student {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateStudent {
    pub full_name: String,
    pub email: String,
}

pub type UpdateStudent = CreateStudent;
