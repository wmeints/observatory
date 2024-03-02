use axum::Router;
use axum::routing::get;

/// Renders the homepage.
async fn index() -> &'static str {
    "Hello from the homepage"
}

/// Creates the router for the front page.
pub fn router() -> Router {
    Router::new().route("/", get(index))
}