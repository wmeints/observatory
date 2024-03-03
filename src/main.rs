

use clap::Parser;

use tokio::net::TcpListener;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    #[arg(short, long, default_value_t = 3000)]
    port: i32,

    #[arg(short, long, default_value = "mongodb://localhost:27017/observatory")]
    database_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let binding_address = format!("{}:{}", args.address, args.port);

    tracing_subscriber::fmt::init();

    let app = observatory::routes::router();
    let listener = TcpListener::bind(binding_address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.unwrap()
}
