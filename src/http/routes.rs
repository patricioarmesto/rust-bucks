use axum::{Router, routing::get, routing::post};

use crate::http::handlers::{health, metrics, orders, ping};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/metrics", get(metrics::metrics))
        .route("/ping", get(ping::ping))
        .route("/orders", post(orders::create_order))
}
