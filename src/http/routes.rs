use axum::{Router, routing::get};

use crate::http::handlers::{health, metrics, ping};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/metrics", get(metrics::metrics))
        .route("/ping", get(ping::ping))
}
