use crate::AppState;
use axum::extract::{Json, State};
use entity::{bin, received_good};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};
use serde_json::json;
use uuid::Uuid;
// TODO! реализовать здесь, на этом месте функцию, возвращающую Router::new().route(...)... для функций по пути /api/bin/{функция}

pub(crate) async fn allocate(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // TODO! поменять возвращаемое значение

    // Проверка на наличие такого UUID
    let good_id = payload["good_id"].as_str().unwrap(); // TODO! сделать обработку ошибок
    let good_uuid = Uuid::parse_str(good_id).unwrap();

    let res = state
        .db
        .transaction::<_, (i32, i32, Uuid), DbErr>(|txn| {
            Box::pin(async move {
                let received_good = match received_good::Entity::find_by_id(good_uuid)
                    .one(txn)
                    .await
                    .unwrap() // TODO! сделать обработку ошибок
                {
                    Some(received_good) => received_good,
                    None => panic!("Good not found!"), // TODO! заменить панику на Err(тип ошибки)
                };

                let free_bin = bin::Entity::find()
                    .filter(bin::Column::Good.is_null())
                    .lock_exclusive()
                    .one(txn)
                    .await
                    .unwrap()
                    .unwrap(); // TODO! сделать обработку ошибок

                let mut bin_active: bin::ActiveModel = free_bin.into();
                bin_active.good = Set(Some(good_uuid));
                bin_active.status = Set(1);

                let bin = bin_active.update(txn).await.unwrap(); // TODO! сделать обработку ошибок

                let delete_res = received_good.delete(txn).await.unwrap();
                tracing::info!(
                    "Deleting received_good, row affected: {}",
                    delete_res.rows_affected
                );

                Ok((bin.id, bin.rack, bin.good.unwrap()))
            })
        })
        .await
        .unwrap();

    tracing::info!(
        "Allocated good, id {}, bin id {}, rack {}",
        res.2,
        res.0,
        res.1
    );
    Json(json!({
        "id": res.0,
        "rack": res.1,
    }))
}

pub(crate) async fn good_arrived(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let good_id = payload["good_id"].as_str().unwrap(); // TODO! сделать обработку ошибок
    let good_uuid = Uuid::parse_str(good_id).unwrap();
    let bin = match bin::Entity::find()
        .filter(bin::Column::Good.eq(good_uuid))
        .filter(bin::Column::Status.eq(1)) // TODO! по-хорошему сделать поверку на целостность памяти, т.к. status не может быть не равен 1
        .one(&state.db) // возможно и несколько результатов, TODO! позже сделать проверку на целостность памяти
        .await
        .unwrap()
    {
        Some(bin) => bin,
        None => panic!("Good in bin not found!"), // TODO! обязательно обработать ошибку
    };

    let mut bin_active: bin::ActiveModel = bin.into();
    bin_active.status = Set(2);

    let bin = bin_active.update(&state.db).await.unwrap();
    tracing::info!(
        "Status changed to {} for good {}",
        bin.status,
        bin.good.unwrap()
    );
    // TODO! возможно поменять возвращаемый JSON
    Json(json!({
        "good_id": bin.good
    }))
}
