use course_api::AppState;
use sqlx::PgPool;
use std::sync::Arc;

mod db;
mod entities;

mod routes;
// use routes::create_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => p,
        Err(err) => {
            println!("Error: {:#?}", err);
            todo!()
        }
    };

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let shared_state = Arc::new(AppState { pool });

    // let app = create_app(shared_state).await?;

    let address = "127.0.0.1:3011";
    println!("Running server at http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    // axum::serve(listener, app).await?;

    Ok(())
}
