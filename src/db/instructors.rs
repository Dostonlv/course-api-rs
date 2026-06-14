use sqlx::{Error, PgPool};

use crate::entities::{
    instructors::{CreateInstructor, Instructor, UpdateInstructor},
    repository::Repository,
};

pub struct InstructorRepo;

impl Repository for InstructorRepo {
    type Model = Instructor;
    type CreateDto = CreateInstructor;
    type UpdateDto = UpdateInstructor;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"INSERT INTO instructors (full_name, email, expertise)
            VALUES ($1, $2, $3) RETURNING id"#,
            dto.full_name,
            dto.email,
            dto.expertise
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error> {
        sqlx::query_as!(
            Instructor,
            r#"SELECT id, full_name, email, expertise 
            FROM instructors WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error> {
        sqlx::query_as!(
            Instructor,
            r#"SELECT id, full_name, email, expertise 
            FROM instructors"#
        )
        .fetch_all(pool)
        .await
    }

    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"UPDATE instructors
            SET full_name = $1, email = $2, expertise = $3
            WHERE id = $4 RETURNING id"#,
            dto.full_name,
            dto.email,
            dto.expertise,
            id
        )
        .fetch_one(pool)
        .await
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(r#"DELETE FROM instructors WHERE id = $1 RETURNING id"#, id)
            .fetch_optional(pool)
            .await
    }
}
