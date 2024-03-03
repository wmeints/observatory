use crate::shared::state::AppState;
use axum::extract::State;
use axum::routing::get;
use axum::Router;

/// Renders the homepage.
pub async fn homepage(State(_state): State<AppState>) -> &'static str {
    "Hello from the homepage"
}

/// Creates the router for the front page.
pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(homepage))
        .with_state(app_state)
}
