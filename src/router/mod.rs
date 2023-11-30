use axum::Router;

pub fn create_main_router() -> Router {
    Router::new().layer(tower_http::trace::TraceLayer::new_for_http())
}
