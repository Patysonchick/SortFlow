pub(crate) mod bin;
pub(crate) mod good;

use crate::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use uuid::Uuid;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .nest("/good", good::routes())
        .nest("/bin", bin::routes())
}

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("Database transaction error: {0}")]
    DatabaseTransaction(String), // (sea_orm::TransactionError<Error>)

    #[error("Good with UUID {0} not found")]
    GoodNotFound(Uuid),

    #[error("UUID not provided")]
    EmptyUUID,

    #[error("UUID parse error: {0}")]
    UUIDParse(#[from] uuid::Error),

    #[error("No free bins available")]
    FreeBinNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("API Error: {}", self);

        let (status, message) = match self {
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error"),
            Error::DatabaseTransaction(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database transaction error",
            ),
            Error::GoodNotFound(_) => (StatusCode::NOT_FOUND, "Good with UUID not found"),
            Error::EmptyUUID => (StatusCode::BAD_REQUEST, "UUID not provided"),
            Error::UUIDParse(_) => (StatusCode::BAD_REQUEST, "UUID parse error"), // возможно надо поменять с bad request на internal error
            Error::FreeBinNotFound => (StatusCode::GONE, "No free bins available"), // возможно надо поменять gone на что-то другое
        };

        (status, message).into_response()
    }
}
