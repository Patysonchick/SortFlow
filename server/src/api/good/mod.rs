use crate::{AppState, api};
use axum::Router;
use axum::extract::{Json, Path, State};
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use entity::received_good;
use image::Luma;
use qrcode::QrCode;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::json;
use std::io::Cursor;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/qrcode/{good_id}", get(qrcode))
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
    headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());

    Ok((headers, bytes))
}
