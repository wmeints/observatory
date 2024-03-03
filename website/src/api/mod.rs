use crate::shared::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, Router},
    Json,
};
use entity::prelude::*;
use sea_orm::EntityTrait;

pub enum FindPostError {
    InternalServerError,
    NotFound,
}

impl IntoResponse for FindPostError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FindPostError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            FindPostError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
        }
    }
}

pub async fn get_posts(
    State(state): State<AppState>,
) -> crate::shared::error::Result<Json<Vec<entity::post::Model>>> {
    let results: Vec<entity::post::Model> = Post::find().all(&state.connection).await?;
    Ok(Json(results))
}

pub async fn get_post_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> anyhow::Result<Json<entity::post::Model>, FindPostError> {
    // Let's try to find a single post.
    // Make sure we map any error here into an internal server problem.
    let result = Post::find_by_id(id)
        .one(&state.connection)
        .await
        .map_err(|e| {
            tracing::error!("Failed to retrieve posts: {:?}", e);
            FindPostError::InternalServerError
        })?;

    // If we get a none here, we haven't found the post, and we should return a 404.
    match result {
        Some(post) => Ok(Json(post)),
        None => Err(FindPostError::NotFound),
    }
}

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(get_posts))
        .route("/api/posts/:id", get(get_post_by_id))
        .with_state(app_state)
}
