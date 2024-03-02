use clap::Parser;
use tracing;
use tracing_subscriber;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(version)]
struct Args{
    #[arg(short,long, default_value = "0.0.0.0")]
    address: String,

    #[arg(short, long, default_value_t = 3000)]
    port: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let binding_address = format!("{}:{}", args.address, args.port);

    tracing_subscriber::fmt::init();

    tracing::info!("Listening on http://{}", binding_address);

    let app = observatory::routes::router();
    let listener = tokio::net::TcpListener::bind(binding_address).await.unwrap();

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