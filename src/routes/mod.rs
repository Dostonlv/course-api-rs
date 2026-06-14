use std::sync::Arc;

use crate::AppState;
use crate::entities::{
    courses::{Course, CreateCourse},
    enrollments::{CreateEnrolment, Enrolment},
    instructors::{CreateInstructor, Instructor},
    students::{CreateStudent, Student},
};
use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod courses;
pub mod enrollments;
pub mod instructors;
pub mod students;

#[derive(OpenApi)]
#[openapi(
    paths(
        students::create_student,
        students::get_student,
        students::get_all,
        students::update_student,
        students::delete_student,
        instructors::create_instructor,
        instructors::get_instructor,
        instructors::get_all,
        instructors::update_instructor,
        instructors::delete_instructor,
        courses::create_course,
        courses::get_course,
        courses::get_all,
        courses::update_course,
        courses::delete_course,
        enrollments::create_enrolment,
        enrollments::get_enrolment,
        enrollments::get_all,
        enrollments::update_enrolment,
        enrollments::delete_enrolment,
    ),
    components(schemas(
        Student,
        CreateStudent,
        Instructor,
        CreateInstructor,
        Course,
        CreateCourse,
        Enrolment,
        CreateEnrolment,
    ))
)]
pub struct ApiDoc;

pub async fn create_app(pool: Arc<AppState>) -> anyhow::Result<Router> {
    let router = Router::new()
        .nest("/courses", courses::router())
        .nest("/students", students::router())
        .nest("/instructors", instructors::router())
        .nest("/enrollments", enrollments::router())
        .with_state(pool)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

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
