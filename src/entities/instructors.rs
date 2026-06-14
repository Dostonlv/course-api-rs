use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, ToSchema, Default)]
pub struct Instructor {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub expertise: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInstructor {
    pub full_name: String,
    pub email: String,
    pub expertise: Option<String>,
}

pub type UpdateInstructor = CreateInstructor;
