use axum::Router;
use clap::{Arg, Command};

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use observatory::shared::state::AppState;
use sea_orm::Database;
use tokio::net::TcpListener;
use tracing_subscriber;

struct CommandLineArgs {
    address: String,
    port: i32,
    database_url: String,
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    let binding_address = format!("{}:{}", args.address, args.port);

    tracing_subscriber::fmt::init();

    let database_connection = Database::connect(args.database_url)
        .await
        .expect("Couldn't connect to the database.");

    // Automatically migrate the database.
    // This saves me a ton of time managing the website :-)
    Migrator::up(&database_connection, None)
        .await
        .expect("Unable to update the database");

    // The app state contains the shared resources for the website such as database connections.
    // The application crashes when we can't construct the application state.
    let app_state = AppState::new(database_connection);

    // The routes for the website are split in two areas: frontpage, and admin.
    // Each has its own module in the application code. We merge the routes into a single router here.
    let app = Router::new()
        .merge(observatory::api::routes(app_state.clone()))
        .merge(observatory::admin::routes(app_state.clone()))
        .merge(observatory::frontpage::routes(app_state.clone()))
        .with_state(app_state);

    let listener = TcpListener::bind(&binding_address)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Listening on {}", &binding_address);

    axum::serve(listener, app).await.unwrap()
}

fn cli() -> Command {
    let port_argument = Arg::new("port")
        .long("port")
        .short('p')
        .default_value("3000");

    let address_argument = Arg::new("address")
        .long("address")
        .short('a')
        .default_value("0.0.0.0");

    let database_url_argument = Arg::new("database_url")
        .long("database-url")
        .short('d')
        .env("DATABASE_URL");

    Command::new("observatory")
        .arg(port_argument)
        .arg(address_argument)
        .arg(database_url_argument)
}

fn parse_args() -> CommandLineArgs {
    dotenv().ok();

    let matches = cli().get_matches();

    let address = matches.get_one::<String>("address").unwrap();
    let port: i32 = matches
        .get_one::<String>("port")
        .map(|port| port.parse::<i32>().unwrap())
        .expect("Port is invalid");

    let database_url = matches
        .get_one::<String>("database_url")
        .expect("Database URL is invalid");

    CommandLineArgs {
        address: address.to_owned(),
        port: port.to_owned(),
        database_url: database_url.to_owned(),
    }
}
