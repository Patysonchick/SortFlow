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
    let listen = env::var("SERVER_LISTEN").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = match env::var("SERVER_PORT") {
        Ok(port) => port
            .parse::<u16>()
            .inspect_err(|_| tracing::error!("PORT is not a number"))?,
        Err(_) => 3000,
    };
    let postgres_listen = env::var("POSTGRES_LISTEN")
        .inspect_err(|_| tracing::error!("POSTGRES_LISTEN not stated"))?;
    let postgres_port =
        env::var("POSTGRES_PORT").inspect_err(|_| tracing::error!("POSTGRES_PORT not stated"))?;
    let postgres_user =
        env::var("POSTGRES_USER").inspect_err(|_| tracing::error!("POSTGRES_USER not stated"))?;
    let postgres_password = env::var("POSTGRES_PASSWORD")
        .inspect_err(|_| tracing::error!("POSTGRES_PASSWORD not stated"))?;
    let postgres_db =
        env::var("POSTGRES_DB").inspect_err(|_| tracing::error!("POSTGRES_DB not stated"))?;

    tracing::info!("Establishing database connection...");
    let url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_listen}:{postgres_port}/{postgres_db}"
    );
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

    tracing::info!("Listening on {listen}:{port}");
    let listener = tokio::net::TcpListener::bind((listen, port)).await?;
    axum::serve(listener, app).await?;

    tracing::info!("Server stopped!");
    Ok(())
}
