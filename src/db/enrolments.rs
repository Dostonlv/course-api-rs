use sqlx::{Error, PgPool};

use crate::entities::{
    courses::Course,
    enrollments::{CreateEnrolment, Enrolment, UpdateEnrolment},
    instructors::Instructor,
    repository::Repository,
    students::Student,
};

pub struct EnrolmentRepo;

impl Repository for EnrolmentRepo {
    type Model = Enrolment;
    type CreateDto = CreateEnrolment;
    type UpdateDto = UpdateEnrolment;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"INSERT INTO enrollments (student_id, course_id) 
            VALUES ($1, $2) RETURNING id"#,
            dto.student_id,
            dto.course_id
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error> {
        let row = sqlx::query!(
            r#"SELECT
                e.id,
                e.enrolled_at,
                s.id as student_id,
                s.full_name as student_full_name,
                s.email as student_email,
                s.created_at as student_created_at,
                c.id as course_id,
                c.title as course_title,
                c.description as course_description,
                c.created_at as course_created_at,
                i.id as instructor_id,
                i.full_name as instructor_full_name,
                i.email as instructor_email,
                i.expertise as instructor_expertise
            FROM enrollments e
            INNER JOIN students s ON e.student_id = s.id
            INNER JOIN courses c ON e.course_id = c.id
            INNER JOIN instructors i ON c.instructor_id = i.id
            WHERE e.id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Enrolment {
            id: r.id,
            enrolled_at: r.enrolled_at,
            student: Student {
                id: r.student_id,
                full_name: r.student_full_name,
                email: r.student_email,
                created_at: r.student_created_at,
            },
            course: Course {
                id: r.course_id,
                title: r.course_title,
                description: r.course_description,
                created_at: r.course_created_at,
                instructor: Instructor {
                    id: r.instructor_id,
                    full_name: r.instructor_full_name,
                    email: r.instructor_email,
                    expertise: r.instructor_expertise,
                },
            },
        }))
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error> {
        let rows = sqlx::query!(
            r#"SELECT
                e.id,
                e.enrolled_at,
                s.id as student_id,
                s.full_name as student_full_name,
                s.email as student_email,
                s.created_at as student_created_at,
                c.id as course_id,
                c.title as course_title,
                c.description as course_description,
                c.created_at as course_created_at,
                i.id as instructor_id,
                i.full_name as instructor_full_name,
                i.email as instructor_email,
                i.expertise as instructor_expertise
            FROM enrollments e
            INNER JOIN students s ON e.student_id = s.id
            INNER JOIN courses c ON e.course_id = c.id
            INNER JOIN instructors i ON c.instructor_id = i.id"#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Enrolment {
                id: r.id,
                enrolled_at: r.enrolled_at,
                student: Student {
                    id: r.student_id,
                    full_name: r.student_full_name,
                    email: r.student_email,
                    created_at: r.student_created_at,
                },
                course: Course {
                    id: r.course_id,
                    title: r.course_title,
                    description: r.course_description,
                    created_at: r.course_created_at,
                    instructor: Instructor {
                        id: r.instructor_id,
                        full_name: r.instructor_full_name,
                        email: r.instructor_email,
                        expertise: r.instructor_expertise,
                    },
                },
            })
            .collect())
    }

    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error> {
        sqlx::query_scalar!(
            r#"UPDATE enrollments 
            SET student_id = $1, course_id = $2
            WHERE id = $3 
            RETURNING id"#,
            dto.student_id,
            dto.course_id,
            id
        )
        .fetch_one(pool)
        .await
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(r#"DELETE FROM enrollments WHERE id = $1 RETURNING id"#, id)
            .fetch_optional(pool)
            .await
    }
}
