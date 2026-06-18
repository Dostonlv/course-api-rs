use course_api::{AppState, config::Config};
use std::sync::Arc;

mod db;
mod entities;

mod routes;
use routes::create_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::new()?;

    let shared_state = Arc::new(AppState::new(config).await?);

    let app = create_app(shared_state).await?;

    let address = "127.0.0.1:3011";
    println!("Running server at http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
