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
