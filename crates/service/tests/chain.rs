//! The chain as an application asks it: allowed only where the chain resolves the flag true.

use std::time::Duration;

use axum::Json;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::get;
use bmo_app_service::{Chain, Subject, allows};
use serde_json::json;

// A stand-in chain: `open` resolves true for person 1 and false for person 2, `quiet` says nothing, and any other
// kind is undeclared.
async fn stand() -> Chain {
    let app = axum::Router::new().route(
        "/chain/kinds/{key}/resolve",
        get(
            |Path(key): Path<String>, Query(query): Query<Vec<(String, String)>>| async move {
                let person = query
                    .iter()
                    .find(|(name, _)| name == "person")
                    .map(|(_, value)| value.clone());

                match (key.as_str(), person.as_deref()) {
                    ("open", Some("1")) => (StatusCode::OK, Json(json!({ "value": true }))),
                    ("open", _) => (StatusCode::OK, Json(json!({ "value": false }))),
                    ("quiet", _) => (StatusCode::OK, Json(json!({}))),
                    _ => (
                        StatusCode::NOT_FOUND,
                        Json(json!({ "code": "no-such-kind" })),
                    ),
                }
            },
        ),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());

    tokio::spawn(async move { axum::serve(listener, app).await });

    Chain::new(&address, "token", "app-a", Duration::from_secs(5))
}

#[tokio::test]
async fn what_the_chain_does_not_allow_is_refused() {
    let chain = stand().await;

    assert!(allows(&chain, "open", Subject::Person(1)).await);
    assert!(!allows(&chain, "open", Subject::Person(2)).await);
    assert!(
        !allows(&chain, "quiet", Subject::Person(1)).await,
        "a flag left unsaid"
    );
    assert!(
        !allows(&chain, "unknown", Subject::Person(1)).await,
        "an undeclared kind"
    );
    assert!(
        !allows(
            &Chain::new("", "", "app-a", Duration::from_secs(1)),
            "open",
            Subject::Person(1)
        )
        .await,
        "no service named"
    );
    assert!(
        !allows(
            &Chain::new("http://127.0.0.1:9", "", "app-a", Duration::from_secs(1)),
            "open",
            Subject::Person(1)
        )
        .await,
        "a service that does not answer"
    );
}
