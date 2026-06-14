use sqlx::{Error, PgPool};

use crate::entities::{
    repository::Repository,
    students::{CreateStudent, Student, UpdateStudent},
};

pub struct StudentRepo;

impl Repository for StudentRepo {
    type Model = Student;
    type CreateDto = CreateStudent;
    type UpdateDto = UpdateStudent;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"INSERT INTO students (full_name, email)
            VALUES ($1, $2) RETURNING id"#,
            dto.full_name,
            dto.email
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error> {
        sqlx::query_as!(
            Student,
            r#"SELECT id, full_name, email, created_at
            FROM students WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error> {
        sqlx::query_as!(
            Student,
            r#"SELECT id, full_name, email, created_at
            FROM students"#
        )
        .fetch_all(pool)
        .await
    }

    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"UPDATE students
            SET full_name = $1, email = $2
            WHERE id = $3 RETURNING id"#,
            dto.full_name,
            dto.email,
            id
        )
        .fetch_one(pool)
        .await
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(r#"DELETE FROM students WHERE id = $1 RETURNING id"#, id)
            .fetch_optional(pool)
            .await
    }
}
