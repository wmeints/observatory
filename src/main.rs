use tracing;
use tracing_subscriber;
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Listening on http://0.0.0.0:3000");

    let app = observatory::routes::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).with_graceful_shutdown(shutdown_handler()).await.unwrap();
}

async fn shutdown_handler() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to create Ctrl+C handler")
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to create signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { tracing::info!("Shutting down") },
        _ = terminate => { tracing::info!("Shutting down") },
    }
}