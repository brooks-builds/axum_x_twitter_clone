mod posts;
use axum::Router;

use crate::state::AppState;

use self::posts::create_posts_router;

pub fn create_main_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/posts", create_posts_router())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
