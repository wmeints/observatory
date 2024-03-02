use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/admin", crate::admin::routes::router())
        .nest("/", crate::frontpage::routes::router())
}