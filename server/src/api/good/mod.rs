use crate::{AppState, api};
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

async fn register(State(state): State<AppState>) -> Result<Json<serde_json::Value>, api::Error> {
    let good = received_good::ActiveModel {
        id: Set(Uuid::now_v7()),
        ..Default::default()
    };

    let good = good.insert(&state.db).await.map_err(api::Error::Database)?;

    tracing::info!("Registered good, id: {}", good.id);
    Ok(Json(json!({
        "good_id": good.id,
    })))
}
