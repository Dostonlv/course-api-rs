use crate::{
    db::enrollments::EnrolmentRepo,
    entities::{
        enrollments::{CreateEnrolment, Enrolment},
        repository::Repository,
    },
    routes::{AppError, AppState, Data},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all).post(create_enrolment))
        .route(
            "/{id}",
            get(get_enrolment)
                .put(update_enrolment)
                .delete(delete_enrolment),
        )
}

#[utoipa::path(
    post,
    path = "/enrollments",
    request_body = CreateEnrolment,
    responses(
        (status = 200, description = "Enrolment created", body = i64),
        (status = 400, description = "Bad request"),
    )
)]
pub async fn create_enrolment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEnrolment>,
) -> Result<Json<i64>, AppError> {
    let id = EnrolmentRepo::create(&state.pool, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(id))
}

#[utoipa::path(
    get,
    path = "/enrollments/{id}",
    params(("id" = i64, Path, description = "Enrolment ID")),
    responses(
        (status = 200, description = "Enrolment found", body = Enrolment),
        (status = 404, description = "Enrolment not found"),
    )
)]
pub async fn get_enrolment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Enrolment>, AppError> {
    let enrolment = EnrolmentRepo::get_by_id(&state.pool, id)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    match enrolment {
        Some(e) => Ok(Json(e)),
        None => Err(AppError {
            status_code: StatusCode::NOT_FOUND,
            data: Json(Data {
                message: String::from("Enrolment not found"),
            }),
        }),
    }
}

#[utoipa::path(
    get,
    path = "/enrollments",
    responses(
        (status = 200, description = "List of enrolments", body = Vec<Enrolment>),
    )
)]
pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Enrolment>>, AppError> {
    let enrolments = EnrolmentRepo::get_all(&state.pool)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(enrolments))
}

#[utoipa::path(
    put,
    path = "/enrollments/{id}",
    params(("id" = i64, Path, description = "Enrolment ID")),
    request_body = CreateEnrolment,
    responses(
        (status = 200, description = "Enrolment updated", body = i64),
        (status = 404, description = "Enrolment not found"),
    )
)]
pub async fn update_enrolment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateEnrolment>,
) -> Result<Json<i64>, AppError> {
    let updated_id = EnrolmentRepo::update(&state.pool, id, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(updated_id))
}

#[utoipa::path(
    delete,
    path = "/enrollments/{id}",
    params(("id" = i64, Path, description = "Enrolment ID")),
    responses(
        (status = 200, description = "Enrolment deleted", body = i64),
        (status = 404, description = "Enrolment not found"),
    )
)]
pub async fn delete_enrolment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<i64>, AppError> {
    let deleted_id = EnrolmentRepo::delete(&state.pool, id)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    match deleted_id {
        Some(id) => Ok(Json(id)),
        None => Err(AppError {
            status_code: StatusCode::NOT_FOUND,
            data: Json(Data {
                message: String::from("Enrolment not found"),
            }),
        }),
    }
}
