use crate::AppState;
use axum::extract::{Json, State};
use entity::received_good;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;

// TODO! реализовать здесь, на этом месте функцию, возвращающую Router::new().route(...)... для функций по пути /api/good/{функция}

pub(crate) async fn register(State(state): State<AppState>) -> Json<serde_json::Value> {
    // TODO! поменять возвращаемое значение
    let good = received_good::ActiveModel {
        id: Set(Uuid::now_v7()),
        ..Default::default()
    };

    let good = good.insert(&state.db).await.unwrap(); // TODO! сделать обработку ошибок

    Json(json!({
        "good_id": good.id,
    }))
}
