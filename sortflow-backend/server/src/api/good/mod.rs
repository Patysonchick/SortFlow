use crate::{AppState, api};
use axum::Router;
use axum::extract::{Json, Path, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use entity::prelude::*;
use entity::{bin, received_good};
use image::Luma;
use qrcode::QrCode;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;
use std::io::Cursor;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/qrcode/{good_id}", get(qrcode))
        .route("/find_in_bin/{good_id}", get(find_in_bin))
}

async fn register(State(state): State<AppState>) -> Result<Json<serde_json::Value>, api::Error> {
    let good = received_good::ActiveModel {
        id: Set(Uuid::now_v7()),
        ..Default::default()
    };

    let good = good.insert(&state.db).await?;

    tracing::info!("Registered good, id: {}", good.id);
    Ok(Json(json!({
        "good_id": good.id,
    })))
}

async fn qrcode(
    // State(state): State<AppState>,
    Path(good_id): Path<Uuid>,
) -> Result<impl IntoResponse, api::Error> {
    // TODO! возможно(но не точно!) сделать проверку на наличие такого UUID

    let code =
        QrCode::new(good_id.to_string().as_bytes()).map_err(|_| api::Error::QrCodeGenerating)?;

    let image = code.render::<Luma<u8>>().build();
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .map_err(|_| api::Error::ImageGenerating)?;

    // TODO! добавить ещё данные на изображение: название сервиса(placeholder - SortFlow), UUID и другие доп данные

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));

    Ok((headers, bytes))
}

async fn find_in_bin(
    State(state): State<AppState>,
    Path(good_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, api::Error> {
    let bin = Bin::find()
        .filter(bin::Column::Good.eq(good_id))
        .filter(bin::Column::Status.eq(2))
        .one(&state.db)
        .await?
        .ok_or(api::Error::GoodNotFound(good_id))?;

    Ok(Json(json!({
        "id": bin.id,
        "rack": bin.rack
    })))
}
