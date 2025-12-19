use crate::{AppState, api};
use axum::Router;
use axum::extract::{Json, State};
use axum::routing::post;
use entity::{bin, received_good};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect, Set,
    TransactionError, TransactionTrait,
};
use serde_json::json;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/allocate", post(allocate))
        .route("/good_arrived", post(good_arrived))
}

async fn allocate(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    // Проверка на наличие такого UUID
    let good_id = payload["good_id"]
        .as_str()
        .ok_or_else(|| api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id).map_err(api::Error::UUIDParse)?;

    let res = state
        .db
        .transaction::<_, (i32, i32, Uuid), api::Error>(|txn| {
            Box::pin(async move {
                let received_good = received_good::Entity::find_by_id(good_uuid)
                    .one(txn)
                    .await
                    .map_err(api::Error::Database)?;
                let received_good = match received_good {
                    Some(received_good) => received_good,
                    None => return Err(api::Error::GoodNotFound(good_uuid)),
                };

                let free_bin = bin::Entity::find()
                    .filter(bin::Column::Good.is_null())
                    .lock_exclusive()
                    .one(txn)
                    .await
                    .map_err(api::Error::Database)?
                    .ok_or_else(|| api::Error::FreeBinNotFound)?;

                let mut bin_active: bin::ActiveModel = free_bin.into();
                bin_active.good = Set(Some(good_uuid));
                bin_active.status = Set(1);

                let bin = bin_active.update(txn).await.map_err(api::Error::Database)?;

                let delete_res = received_good
                    .delete(txn)
                    .await
                    .map_err(api::Error::Database)?;
                tracing::info!(
                    "Deleting received_good, row affected: {}",
                    delete_res.rows_affected
                );

                Ok((
                    bin.id,
                    bin.rack,
                    bin.good.ok_or_else(|| api::Error::EmptyUUID)?,
                ))
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

async fn good_arrived(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, api::Error> {
    let good_id = payload["good_id"]
        .as_str()
        .ok_or_else(|| api::Error::EmptyUUID)?;
    let good_uuid = Uuid::parse_str(good_id).map_err(api::Error::UUIDParse)?;

    let bin = bin::Entity::find()
        .filter(bin::Column::Good.eq(good_uuid))
        .filter(bin::Column::Status.eq(1)) // TODO! по-хорошему сделать поверку на целостность памяти, т.к. status не может быть не равен 1
        .one(&state.db) // возможно и несколько результатов, TODO! позже сделать проверку на целостность памяти
        .await
        .map_err(api::Error::Database)?;
    let bin = match bin {
        Some(bin) => bin,
        None => return Err(api::Error::GoodNotFound(good_uuid)),
    };

    let mut bin_active: bin::ActiveModel = bin.into();
    bin_active.status = Set(2);

    let bin = bin_active
        .update(&state.db)
        .await
        .map_err(api::Error::Database)?;
    tracing::info!(
        "Status changed to {} for good {}",
        bin.status,
        bin.good.ok_or_else(|| api::Error::EmptyUUID)?
    );
    // TODO! возможно поменять возвращаемый JSON
    Ok(Json(json!({
        "good_id": bin.good.ok_or_else(|| api::Error::EmptyUUID)?
    })))
}

// TODO! реализовать убывание груза
