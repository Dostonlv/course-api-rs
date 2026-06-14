use sqlx::{Error, PgPool};


use crate::entities::{
    repository::Repository,
    instructors::{CreateInstructor,Instructor, UpdateInstructor}
};

pub struct InstructorRepo;

impl Repository for InstructorRepo {
    type Model= Instructor;

    type CreateDto=CreateInstructor;

    type UpdateDto=UpdateInstructor;

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