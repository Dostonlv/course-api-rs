use crate::{
    db::instructors::InstructorRepo,
    entities::{
        instructors::{CreateInstructor, Instructor},
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
        .route("/", get(get_all).post(create_instructor))
        .route(
            "/{id}",
            get(get_instructor)
                .put(update_instructor)
                .delete(delete_instructor),
        )
}

#[utoipa::path(
    post,
    path = "/instructors",
    request_body = CreateInstructor,
    responses(
        (status = 200, description = "Instructor created", body = Instructor),
        (status = 400, description = "Bad request"),
    )
)]
pub async fn create_instructor(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInstructor>,
) -> Result<Json<Instructor>, AppError> {
    let id = InstructorRepo::create(&state.pool, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(Instructor {
        id,
        full_name: payload.full_name,
        email: payload.email,
        expertise: payload.expertise,
    }))
}

#[utoipa::path(
    get,
    path = "/instructors/{id}",
    params(("id" = i64, Path, description = "Instructor ID")),
    responses(
        (status = 200, description = "Instructor found", body = Instructor),
        (status = 404, description = "Instructor not found"),
    )
)]
pub async fn get_instructor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Instructor>, AppError> {
    let instructor = InstructorRepo::get_by_id(&state.pool, id)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    match instructor {
        Some(i) => Ok(Json(i)),
        None => Err(AppError {
            status_code: StatusCode::NOT_FOUND,
            data: Json(Data {
                message: String::from("Instructor not found"),
            }),
        }),
    }
}

#[utoipa::path(
    get,
    path = "/instructors",
    responses(
        (status = 200, description = "List of instructors", body = Vec<Instructor>),
    )
)]
pub async fn get_all(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Instructor>>, AppError> {
    let instructors = InstructorRepo::get_all(&state.pool)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(instructors))
}

#[utoipa::path(
    put,
    path = "/instructors/{id}",
    params(("id" = i64, Path, description = "Instructor ID")),
    request_body = CreateInstructor,
    responses(
        (status = 200, description = "Instructor updated", body = Instructor),
        (status = 404, description = "Instructor not found"),
    )
)]
pub async fn update_instructor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateInstructor>,
) -> Result<Json<Instructor>, AppError> {
    let updated_id = InstructorRepo::update(&state.pool, id, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(Instructor {
        id: updated_id,
        full_name: payload.full_name,
        email: payload.email,
        expertise: payload.expertise,
    }))
}

#[utoipa::path(
    delete,
    path = "/instructors/{id}",
    params(("id" = i64, Path, description = "Instructor ID")),
    responses(
        (status = 200, description = "Instructor deleted", body = i64),
        (status = 404, description = "Instructor not found"),
    )
)]
pub async fn delete_instructor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<i64>, AppError> {
    let deleted_id = InstructorRepo::delete(&state.pool, id)
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
                message: String::from("Instructor not found"),
            }),
        }),
    }
}
