use axum::Router;
use axum::routing::get;

async fn index() -> &'static str {
    "Hello from the homepage"
}

pub fn router() -> Router {
    Router::new().route("/", get(index))
}