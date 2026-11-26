//! The serving loop: every application answers on the address its unit gives it and stops
//! when the terminal or the service manager asks it to, in time for its stop to look like
//! a stop and not a kill.

use axum::Router;
use tokio::net::TcpListener;

/// Serves the router on the listener until [`shutdown_signal`] ends, finishing the answers
/// already under way before returning.
pub async fn serve(listener: TcpListener, app: Router) -> std::io::Result<()> {
    serve_until(listener, app, shutdown_signal()).await
}

/// The same loop ended by a stop the caller hands in, for an application that closes its own
/// event streams the moment the stop begins and before the last answers leave.
pub async fn serve_until(
    listener: TcpListener,
    app: Router,
    stop: impl std::future::Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    axum::serve(listener, app)
        .with_graceful_shutdown(stop)
        .await
}

/// Ends on an interrupt or on the stop signal the service manager sends.
pub async fn shutdown_signal() {
    let Ok(mut terminate) =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
    else {
        let _ = tokio::signal::ctrl_c().await;
        return;
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = terminate.recv() => {}
    }
}
