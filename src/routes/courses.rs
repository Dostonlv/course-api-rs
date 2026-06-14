use crate::{
    db::students::StudentRepo,
    entities::{
        repository::Repository,
        students::{CreateStudent, Student},
    },
    routes::{AppError, AppState, Data},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all).post(create_student))
        .route(
            "/{id}",
            get(get_student).put(update_student).delete(delete_student),
        )
}

pub async fn create_student(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateStudent>,
) -> Result<Json<Student>, AppError> {
    let id = StudentRepo::create(&state.pool, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(Student {
        id,
        full_name: payload.full_name,
        email: payload.email,
        created_at: None,
    }))
}

pub async fn get_student(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Student>, AppError> {
    let student = StudentRepo::get_by_id(&state.pool, id)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    match student {
        Some(s) => Ok(Json(s)),
        None => Err(AppError {
            status_code: StatusCode::NOT_FOUND,
            data: Json(Data {
                message: String::from("Student not found"),
            }),
        }),
    }
}

pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Student>>, AppError> {
    let students = StudentRepo::get_all(&state.pool)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(students))
}

pub async fn update_student(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateStudent>,
) -> Result<Json<Student>, AppError> {
    let updated_id = StudentRepo::update(&state.pool, id, &payload)
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::BAD_REQUEST,
            data: Json(Data {
                message: format!("{:?}", err),
            }),
        })?;

    Ok(Json(Student {
        id: updated_id,
        full_name: payload.full_name,
        email: payload.email,
        created_at: None,
    }))
}

pub async fn delete_student(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<i64>, AppError> {
    let deleted_id = StudentRepo::delete(&state.pool, id)
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
                message: String::from("Student not found"),
            }),
        }),
    }
}
