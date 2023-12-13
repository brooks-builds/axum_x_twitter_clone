mod posts;
use axum::{http::header::CONTENT_TYPE, http::Method, Router};
use tower_http::cors::CorsLayer;

use crate::state::AppState;

use self::posts::create_posts_router;

pub fn create_main_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(["http://localhost:8080".parse().unwrap()]);

    Router::new()
        .nest("/api/v1/posts", create_posts_router())
        .with_state(state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
