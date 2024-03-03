use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

/// Creates the router for the pages under /admin.
pub fn router() -> Router {
    let assets = ServeDir::new("./admin/dist/assets");
    let index_file = ServeFile::new("./admin/dist/index.html");

    // This routes the assets path to the static assets folder in the admin frontend app.
    // When the user accesses a route that doesn't exist we'll route the request to index.html.
    Router::new()
        .route_service(
            "/assets",
            get_service(assets).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }),
        )
        .fallback(get_service(index_file).handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }))
}
