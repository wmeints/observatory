use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::shared::state::AppState;

/// Creates the router for the pages under /admin.
pub fn routes(app_state: AppState) -> Router<AppState> {
    let single_page_app =
        ServeDir::new("./admin/dist").not_found_service(ServeFile::new("./admin/dist/index.html"));

    // This routes the assets path to the static assets folder in the admin frontend app.
    // When the user accesses a route that doesn't exist we'll route the request to index.html.
    Router::new()
        .nest_service("/admin", single_page_app.clone())
        .with_state(app_state)
}
