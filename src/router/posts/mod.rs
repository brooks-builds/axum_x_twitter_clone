use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub mod create;
pub mod get_all_top_level;
pub mod get_one;

pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_post))
        .route("/", get(get(get_all_top_level::get_all_top_level)))
        .route("/:id", get(get_one::get_one_post))
}
