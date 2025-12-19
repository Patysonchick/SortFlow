pub(crate) mod bin;
pub(crate) mod good;

use crate::AppState;
use axum::Router;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .nest("/good", good::routes())
        .nest("/bin", bin::routes())
}

// TODO! сделать тип данных для ошибок
