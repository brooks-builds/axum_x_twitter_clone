use crate::state::AppState;
use axum::{routing::post, Router};

pub mod create;

pub fn create_posts_router() -> Router<AppState> {
    Router::new().route("/", post(create::create_post))
}
