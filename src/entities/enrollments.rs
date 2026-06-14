use crate::entities::{courses::Course, students::Student};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, ToSchema)]
pub struct Enrolment {
    pub id: i64,
    pub enrolled_at: Option<chrono::NaiveDateTime>,
    pub student: Student,
    pub course: Course,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEnrolment {
    pub student_id: i64,
    pub course_id: i64,
}

pub type UpdateEnrolment = CreateEnrolment;
