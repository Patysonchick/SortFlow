use crate::{AppState, api};
use axum::Router;
use axum::extract::{Json, State};
use axum::routing::post;
use entity::{bin, received_good};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
    Set, TransactionError, TransactionTrait,
};
use serde_json::json;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/allocate", post(allocate))
        .route("/arrived", post(arrived))
        .route("/pick", post(pick))
        .route("/departed", post(departed))
}

async fn allocate(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    // Проверка на наличие такого UUID
    let good_id = payload["good_id"].as_str().ok_or(api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id)?;

    let res = state
        .db
        .transaction::<_, (i32, i32, Uuid), api::Error>(|txn| {
            Box::pin(async move {
                let received_good = received_good::Entity::find_by_id(good_uuid)
                    .one(txn)
                    .await?
                    .ok_or(api::Error::GoodNotFound(good_uuid))?;

                let free_bin = bin::Entity::find()
                    .filter(bin::Column::Good.is_null())
                    .filter(bin::Column::Status.eq(0))
                    .order_by_asc(bin::Column::Id)
                    // .order_by_asc(bin::Column::Rack) // возможно можно добавить сортировку по стеллажу
                    .lock_exclusive()
                    .one(txn)
                    .await?
                    .ok_or(api::Error::FreeBinNotFound)?;
                // TODO! сделать сортировку между стеллажами по степени нагруженности манипулятора
                // наверное надо будет распределять по алгоритму Round Robin(каждому по очереди: 1, 2, 3,  1, 2, ...)

                let mut bin_active: bin::ActiveModel = free_bin.into();
                bin_active.good = Set(Some(good_uuid));
                bin_active.status = Set(1);

                let bin = bin_active.update(txn).await?;

                let delete_res = received_good.delete(txn).await?;
                tracing::info!(
                    "Deleting received_good, row affected: {}",
                    delete_res.rows_affected
                );

                Ok((bin.id, bin.rack, good_uuid))
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(err) => api::Error::Database(err),
            TransactionError::Transaction(err) => err,
        })?;

    tracing::info!(
        "Allocated good, id {}, bin id {}, rack {}",
        res.2,
        res.0,
        res.1
    );
    Ok(Json(json!({
        "id": res.0,
        "rack": res.1,
    })))
}

async fn arrived(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    let good_id = payload["good_id"].as_str().ok_or(api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id)?;

    let bin = bin::Entity::find()
        .filter(bin::Column::Good.eq(good_uuid))
        .filter(bin::Column::Status.eq(1)) // TODO! по-хорошему сделать поверку на целостность памяти, т.к. status не может быть не равен 1
        .one(&state.db) // возможно и несколько результатов, TODO! позже сделать проверку на целостность памяти
        .await?
        .ok_or(api::Error::GoodNotFound(good_uuid))?;

    let mut bin_active: bin::ActiveModel = bin.into();
    bin_active.status = Set(2);

    let bin = bin_active.update(&state.db).await?;

    tracing::info!(
        "Status changed to {} for good {}",
        bin.status,
        bin.good.ok_or(api::Error::EmptyUUID)?
    );
    // TODO! возможно поменять возвращаемый JSON
    Ok(Json(json!({
        "good_id": bin.good.ok_or(api::Error::EmptyUUID)?
    })))
}

async fn pick(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    let good_id = payload["good_id"].as_str().ok_or(api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id)?;

    let bin = bin::Entity::find()
        .filter(bin::Column::Good.eq(good_uuid))
        .filter(bin::Column::Status.eq(2))
        .one(&state.db)
        .await?
        .ok_or(api::Error::GoodNotFound(good_uuid))?;

    let mut bin_active: bin::ActiveModel = bin.into();
    bin_active.status = Set(3);

    let bin = bin_active.update(&state.db).await?;

    tracing::info!(
        "Status changed to {} for good {}",
        bin.status,
        bin.good.ok_or(api::Error::EmptyUUID)?
    );
    Ok(Json(json!({
        "good_id": bin.good.ok_or(api::Error::EmptyUUID)?
    })))
}

async fn departed(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    let good_id = payload["good_id"].as_str().ok_or(api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id)?;

    let bin = bin::Entity::find()
        .filter(bin::Column::Good.eq(good_uuid))
        .filter(bin::Column::Status.eq(3))
        .one(&state.db)
        .await?
        .ok_or(api::Error::GoodNotFound(good_uuid))?;

    let mut bin_active: bin::ActiveModel = bin.into();
    bin_active.good = Set(None);
    bin_active.status = Set(0);

    let bin = bin_active.update(&state.db).await?;

    tracing::info!("Deleting good {} from bin {}", good_uuid, bin.id);
    Ok(Json(json!({
        "good_id": good_uuid
    })))
}
