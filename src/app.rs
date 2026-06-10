use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

use crate::{
    handlers,
    shop::{self, ShopState},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/healthz", get(handlers::healthz))
        .route("/shop", get(handlers::shop))
        .route("/blog", get(handlers::blog_index))
        .route("/blog/{slug}", get(handlers::blog_post))
        .route("/api/shop/status", get(shop::status))
        .route("/api/shop/mint", post(shop::mint))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(ShopState::default())
}
