//! The JSON envelope: what a success and each kind of failure answer with.

use axum::http::StatusCode;
use axum::response::Response;
use bmo_app_service::{BoxError, answer, not_found, problem};
use http_body_util::BodyExt;
use serde_json::Value;

/// The response's status and its JSON body, read before the body's stream can end.
async fn answered(response: Response) -> (StatusCode, Value) {
    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("the body collects")
        .to_bytes();
    (
        status,
        serde_json::from_slice(&body).expect("every answer is JSON"),
    )
}

#[tokio::test]
async fn a_success_answers_its_own_json() {
    let result: Result<Vec<String>, BoxError> = Ok(vec!["one".into(), "two".into()]);
    let (status, body) = answered(answer(result)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        serde_json::json!(["one", "two"]),
        "a success is its value's JSON, not wrapped"
    );
}

#[tokio::test]
async fn a_failure_answers_500_with_the_problem() {
    let result: Result<(), BoxError> = Err("the store is closed".into());
    let (status, body) = answered(answer(result)).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, serde_json::json!({ "error": "the store is closed" }));
}

#[tokio::test]
async fn the_fallback_answers_404_in_the_same_shape() {
    let (status, body) = answered(not_found()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, serde_json::json!({ "error": "not found" }));
}

#[tokio::test]
async fn a_stated_failure_carries_the_stated_status() {
    let (status, body) = answered(problem(StatusCode::BAD_REQUEST, "the id is missing")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, serde_json::json!({ "error": "the id is missing" }));
}
