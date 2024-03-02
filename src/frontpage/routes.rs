use axum::routing::get;
use axum::Router;

/// Renders the homepage.
async fn index() -> &'static str {
    "Hello world"
}

/// Creates the router for the front page.
pub fn router() -> Router {
    Router::new().route("/", get(index))
}
