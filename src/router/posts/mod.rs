use axum::{routing::post, Router};

use self::create::create_post;

pub mod create;

pub fn create_posts_router() -> Router {
    Router::new().route("/", post(create_post))
}
