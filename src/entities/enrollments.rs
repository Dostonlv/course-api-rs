use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, ToSchema)]
pub struct Enrolment {
    pub id: i64,
    pub student_id: i64,
    pub course_id: i64,
    pub enrolled_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEnrolment {
    pub student_id: i64,
    pub course_id: i64,
    pub enrolled_at: String,
}

pub type UpdateEnrolment = CreateEnrolment;
