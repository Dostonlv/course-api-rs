use sqlx::{Error, PgPool};

use crate::entities::{
    courses::{Course, CreateCourse, UpdateCourse},
    instructors::Instructor,
    repository::Repository,
};

pub struct CourseRepo;

impl Repository for CourseRepo {
    type Model = Course;

    type CreateDto = CreateCourse;

    type UpdateDto = UpdateCourse;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"
        INSERT INTO
        courses (title,description,instructor_id)
         VALUES ($1,$2,$3) RETURNING id"#,
            dto.title,
            dto.description,
            dto.instructor_id
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error> {
        let row = sqlx::query!(
            r#"SELECT 
            c.id, 
            c.title,
            c.description, 
            c.created_at,
            i.id as instructor_id,
            i.full_name as instructor_full_name,
            i.email as instructor_email,
            i.expertise as instructor_expertise
        FROM courses c
        INNER JOIN instructors i ON c.instructor_id = i.id
        WHERE c.id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?;

        let course = row.map(|r| Course {
            id: r.id,
            title: r.title,
            description: r.description,
            created_at: r.created_at,
            instructor: Instructor {
                id: r.instructor_id,
                full_name: r.instructor_full_name,
                email: r.instructor_email,
                expertise: r.instructor_expertise,
            },
        });

        Ok(course)
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error> {
        let rows = sqlx::query!(
            r#"SELECT 
            c.id, 
            c.title,
            c.description, 
            c.created_at,
            i.id as instructor_id,
            i.full_name as instructor_full_name,
            i.email as instructor_email,
            i.expertise as instructor_expertise
        FROM courses c
        INNER JOIN instructors i ON c.instructor_id = i.id"#
        )
        .fetch_all(pool)
        .await?;

        let courses = rows
            .into_iter()
            .map(|r| Course {
                id: r.id,
                title: r.title,
                description: r.description,
                created_at: r.created_at,
                instructor: Instructor {
                    id: r.instructor_id,
                    full_name: r.instructor_full_name,
                    email: r.instructor_email,
                    expertise: r.instructor_expertise,
                },
            })
            .collect();

        Ok(courses)
    }

    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"UPDATE courses 
        SET title = $1, description = $2, instructor_id = $3
        WHERE id = $4 
        RETURNING id"#,
            dto.title,
            dto.description,
            dto.instructor_id,
            id
        )
        .fetch_one(pool)
        .await
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(r#"DELETE FROM courses WHERE id = $1 RETURNING id"#, id)
            .fetch_optional(pool)
            .await
    }
}
