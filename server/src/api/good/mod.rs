use crate::AppState;
use axum::Router;
use axum::extract::{Json, State};
use axum::routing::get;
use entity::received_good;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::json;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/register", get(register))
}

async fn register(State(state): State<AppState>) -> Json<serde_json::Value> {
    // TODO! поменять возвращаемое значение
    let good = received_good::ActiveModel {
        id: Set(Uuid::now_v7()),
        ..Default::default()
    };

    let good = good.insert(&state.db).await.unwrap(); // TODO! сделать обработку ошибок

    tracing::info!("Registered good, id: {}", good.id);
    Json(json!({
        "good_id": good.id,
    }))
}
