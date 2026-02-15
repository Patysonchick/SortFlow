use crate::AppState;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use entity::bin;
use entity::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    rack_id: i32,
    bins: Vec<BinTemplate>,
}

struct BinTemplate {
    pub id: i32,
    pub good: String,
    pub status: i32,
}

pub(crate) async fn index(State(state): State<AppState>, Path(rack_id): Path<i32>) -> Html<String> {
    let bins = Bin::find()
        .filter(bin::Column::Rack.eq(rack_id))
        .order_by_asc(bin::Column::Id)
        .all(&state.db)
        .await
        .unwrap();

    let bins = bins
        .iter()
        .map(|bin| BinTemplate {
            id: bin.id,
            good: bin.good.map(|g| g.to_string()).unwrap_or_default(),
            status: bin.status,
        })
        .collect();

    // TODO! дописать логику при переполнении rack_id (ввод не существующего стеллажа)

    Html(IndexTemplate { rack_id, bins }.render().unwrap())
}
