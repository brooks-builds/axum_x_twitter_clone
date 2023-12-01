mod posts;
use axum::Router;

use self::posts::create_posts_router;

pub fn create_main_router() -> Router {
    Router::new()
        .nest("/api/v1/posts", create_posts_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
