use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(crate::admin::routes::router())
        .merge(crate::frontpage::routes::router())
}
