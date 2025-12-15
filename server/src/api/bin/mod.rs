use crate::AppState;
use axum::extract::{Json, State};
use entity::{bin, received_good};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

// TODO! реализовать здесь, на этом месте функцию, возвращающую Router::new().route(...)... для функций по пути /api/bin/{функция}

pub(crate) async fn allocate(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO! поменять возвращаемое значение
    // TODO! реализовать логику работы для колонки status

    let good_id = payload["good_id"].as_str().unwrap(); // TODO! сделать обработку ошибок
    let good_uuid = Uuid::parse_str(good_id).unwrap();
    if received_good::Entity::find_by_id(good_uuid)
        .one(&state.db)
        .await
        .unwrap()
        .is_none()
    {
        // TODO! сделать обработку ошибок
        panic!("Good not found!"); // TODO! заменить панику на Err(тип ошибки)
    }

    let free_bin = bin::Entity::find()
        .filter(bin::Column::Good.is_null())
        .one(&state.db)
        .await
        .unwrap()
        .unwrap(); // TODO! сделать обработку ошибок
    let mut bin_active: bin::ActiveModel = free_bin.into();
    bin_active.good = Set(Some(good_uuid));

    let bin = bin_active.update(&state.db).await.unwrap(); // TODO! сделать обработку ошибок
    Json(json!({
        "id": bin.id,
        "rack": bin.rack,
    }))

    // TODO! возможно посже реализовать удаление товара из received_good
}
