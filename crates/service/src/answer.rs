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

/// The body of a failed answer whose consumers read a code with the message.
#[derive(Serialize)]
pub struct CodeProblem {
    pub code: String,
    pub error: String,
}

/// A failure answered with the status its code carries, as `code` and `error` in one body.
///
/// A console application's console, tool and client read a code with every failure, so its
/// handlers answer through this; a plain failure that names no code keeps [`problem`].
pub fn code_problem(
    status: StatusCode,
    code: impl Into<String>,
    error: impl std::fmt::Display,
) -> Response {
    (
        status,
        Json(CodeProblem {
            code: code.into(),
            error: error.to_string(),
        }),
    )
        .into_response()
}

/// A success as its JSON, a failure as a 500 [`Problem`] naming what went wrong.
pub fn answer<T: Serialize>(result: Result<T, BoxError>) -> Response {
    match result {
        Ok(value) => Json(value).into_response(),
        Err(error) => problem(StatusCode::INTERNAL_SERVER_ERROR, error),
    }
}
