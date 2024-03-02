use std::sync::Arc;

use axum::Router;

use crate::shared::state::AppState;

pub fn router(app_state: Arc<AppState>) -> Router {
    let router: Router<_> = Router::new()
        .with_state(app_state)
        .nest("/admin", crate::admin::routes::router())
        .nest("/", crate::frontpage::routes::router());

    router
}
