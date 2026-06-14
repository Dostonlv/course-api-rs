use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, ToSchema)]
pub struct Student {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudent {
    pub full_name: String,
    pub email: String,
}

pub type UpdateStudent = CreateStudent;
