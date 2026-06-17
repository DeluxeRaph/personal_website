use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/error", get(handlers::error_redirect))
        .route("/healthz", get(handlers::healthz))
        .route("/blog", get(handlers::blog_index))
        .route("/blog/{slug}", get(handlers::blog_post))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handlers::not_found)
}
