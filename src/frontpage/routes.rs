use crate::shared::error::AppError;
use crate::shared::models::Post;
use anyhow::Result;
use axum::routing::get;
use axum::{Json, Router};

/// Renders the homepage.
pub async fn index() -> Result<Json<Vec<Post>>, AppError> {
    let posts: Vec<Post> = vec![];
    Ok(Json(posts))
}

/// Creates the router for the front page.
pub fn router() -> Router {
    Router::new().route("/", get(index))
}
