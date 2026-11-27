//! The JSON envelope: what a success and each kind of failure answer with.

use axum::http::StatusCode;
use axum::response::Response;
use bmo_app_service::{
    BoxError, Notice, Refusal, answer, code_problem, not_found, problem, refusal,
};
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

#[tokio::test]
async fn a_coded_failure_carries_code_and_status_together() {
    let (status, body) = answered(code_problem(
        StatusCode::CONFLICT,
        "version-conflict",
        "the note stands at version 3",
    ))
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(
        body,
        serde_json::json!({ "code": "version-conflict", "error": "the note stands at version 3" }),
        "console-family consumers read the code beside the message"
    );
}

#[tokio::test]
async fn a_refusal_says_who_may_and_what_to_do_next() {
    let (status, body) = answered(refusal(
        StatusCode::FORBIDDEN,
        Refusal {
            code: "not-a-writer".into(),
            error: "Only the calendar's writers change its events.".into(),
            who_may: Some("the administrator and the calendar's writers".into()),
            next: Some("ask a writer to add you".into()),
        },
    ))
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        body,
        serde_json::json!({
            "code": "not-a-writer",
            "error": "Only the calendar's writers change its events.",
            "whoMay": "the administrator and the calendar's writers",
            "next": "ask a writer to add you",
        })
    );

    let (_, bare) = answered(refusal(
        StatusCode::FORBIDDEN,
        Refusal {
            code: "forbidden".into(),
            error: "No.".into(),
            who_may: None,
            next: None,
        },
    ))
    .await;
    assert_eq!(
        bare,
        serde_json::json!({ "code": "forbidden", "error": "No." })
    );
}

#[test]
fn a_notice_reads_its_fact_its_reason_and_its_next_step() {
    let notice = Notice {
        fact: "The booking of room 3 was moved to 15:00.".into(),
        next: "tell the people booked".into(),
        why: Some("The room was taken for a repair.".into()),
    };
    assert_eq!(
        notice.text(),
        "The booking of room 3 was moved to 15:00.\nThe room was taken for a repair.\nNext: tell the people booked"
    );
    assert_eq!(
        Notice {
            why: None,
            ..notice
        }
        .text(),
        "The booking of room 3 was moved to 15:00.\nNext: tell the people booked"
    );
}
