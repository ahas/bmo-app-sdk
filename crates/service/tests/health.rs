//! The health checks against a service that answers, and against no service at all.

use axum::Json;
use axum::Router;
use axum::routing::get;
use bmo_app_service::{health_check, site_check};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

/// A service answering 200 at `/api/health` and at the root, on a port of the loopback's
/// own, kept down by the returned sender: the test ends the server before it ends.
async fn standing() -> (String, oneshot::Sender<()>) {
    let app = Router::new()
        .route(
            "/api/health",
            get(|| async { Json(serde_json::json!({ "ok": true })) }),
        )
        .route("/", get(|| async { "the root page" }));
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let address = listener.local_addr().expect("the bound address");
    let (stop, stopped) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await;
    });
    (format!("http://{address}/api/health"), stop)
}

#[tokio::test]
async fn the_check_passes_where_the_health_route_answers() {
    let (url, stop) = standing().await;
    health_check(&url).await.expect("the health answers");
    stop.send(()).expect("the server is still listening");
}

#[tokio::test]
async fn the_site_check_passes_where_health_and_the_root_answer() {
    let (url, stop) = standing().await;
    site_check(&url)
        .await
        .expect("the site answers at both addresses");
    stop.send(()).expect("the server is still listening");
}

#[tokio::test]
async fn a_check_fails_where_nothing_listens() {
    // Bound and dropped at once: the port stands free and refuses every connection.
    let address = {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("an ephemeral port");
        listener.local_addr().expect("its address")
    };
    let error = health_check(&format!("http://{address}/api/health"))
        .await
        .expect_err("nothing answers on the free port");
    assert!(!error.to_string().is_empty(), "the failure names its cause");
}
