pub(crate) mod api;

use axum::Router;
use axum::routing::get;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => tracing::info!("Found .env file"),
        Err(e) => tracing::warn!("{e}\nFailed reading .env file, using default env vars"),
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!("Starting server...");

    // TODO! сделать подключение не по ссылке, а по отдельным данным для входа
    // TODO! изменить структуру .env файла
    let url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("DATABASE_URL not found");
            return Err(e.into());
        }
    };
    let db = Database::connect(url).await?;
    tracing::info!("Database connection established");

    tracing::info!("Checking for new migrations...");
    Migrator::up(&db, None).await?;
    tracing::info!("Migrations applied successfully!");

    let state = AppState { db };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::routes())
        .with_state(state);

    // TODO! сообщение о начале прослушивания на порте
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    tracing::info!("Server stopped!");
    Ok(())
}
