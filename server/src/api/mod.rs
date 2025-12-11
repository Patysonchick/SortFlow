use crate::AppState;
use axum::extract::{Json, State};
use entity::received_good;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::json;
use uuid::Uuid;

// , Json(payload): Json<serde_json::Value>
pub async fn register_good(State(state): State<AppState>) -> Json<serde_json::Value> {
    let good = received_good::ActiveModel {
        id: Set(Uuid::now_v7()),
        ..Default::default()
    };

    let good = good.insert(&state.db).await.unwrap(); // TODO! сделать обработку ошибок

    Json(json!({
        "id": good.id,
    }))
}
