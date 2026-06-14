use sqlx::{Error, PgPool};

pub trait Repository {
    type Model;
    type CreateDto;
    type UpdateDto;

    async fn create(pool: &PgPool, dto: &Self::CreateDto) -> Result<i64, Error>;
    async fn get_by_id(pool: &PgPool, id: i64) -> Result<Option<Self::Model>, Error>;
    async fn get_all(pool: &PgPool) -> Result<Vec<Self::Model>, Error>;
    async fn update(pool: &PgPool, id: i64, dto: &Self::UpdateDto) -> Result<i64, Error>;
    async fn delete(pool: &PgPool, id: i64) -> Result<Option<i64>, Error>;
}
