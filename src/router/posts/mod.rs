pub mod create;
pub mod get_all_top_level;
pub mod get_one;
mod update;

use crate::state::AppState;
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_post))
        .route("/", get(get(get_all_top_level::get_all_top_level)))
        .route("/:id", get(get_one::get_one_post))
        .route("/:id", patch(update::update_post_text))
}
