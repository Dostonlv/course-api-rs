use sqlx::{Error, PgPool};

use crate::entities::{
    courses::Course,
    enrollments::{CreateEnrolment, Enrolment, UpdateEnrolment},
    repository::Repository,
};

pub struct EnrolmentRepo;

impl Repository for EnrolmentRepo {
    type Model = Course;

    type CreateDto = CreateEnrolment;

    type UpdateDto = UpdateEnrolment;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error> {
        todo!()
    }

    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error> {
        todo!()
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error> {
        todo!()
    }

    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error> {
        todo!()
    }

    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error> {
        todo!()
    }
}
