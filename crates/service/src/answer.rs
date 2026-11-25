//! The JSON envelope every failed answer of an application's API carries, and the helpers
//! that put a route handler's result into it.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::BoxError;

/// The body of every failed answer: one `error` string.
#[derive(Serialize)]
pub struct Problem {
    pub error: String,
}

/// A failure answered as a [`Problem`] with the given status.
pub fn problem(status: StatusCode, error: impl std::fmt::Display) -> Response {
    (
        status,
        Json(Problem {
            error: error.to_string(),
        }),
    )
        .into_response()
}

/// The 404 every router's fallback answers, in JSON like every other answer.
pub fn not_found() -> Response {
    problem(StatusCode::NOT_FOUND, "not found")
}

/// A success as its JSON, a failure as a 500 [`Problem`] naming what went wrong.
pub fn answer<T: Serialize>(result: Result<T, BoxError>) -> Response {
    match result {
        Ok(value) => Json(value).into_response(),
        Err(error) => problem(StatusCode::INTERNAL_SERVER_ERROR, error),
    }
}
