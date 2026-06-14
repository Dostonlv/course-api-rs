use sqlx::{Pool, Postgres};

mod entities;

pub struct AppState {
    pub pool: Pool<Postgres>,
}