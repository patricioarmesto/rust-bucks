use axum::{Router, middleware, routing::{delete, get, post}};

use crate::http::{
    handlers::{health, metrics, orders::cancel, orders::create, orders::get, ping},
    middleware::{concurrency_limit::ConcurrencyLimitLayer, request_id},
    state::AppState,
};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/metrics", get(metrics::metrics))
        .route("/ping", get(ping::ping))
        .route("/orders", post(create::create_order))
        .route("/orders/:order_id", get(get::get_order))
        .route("/orders/:order_id", delete(cancel::cancel_order))
        .layer(middleware::from_fn(request_id::request_id_middleware))
        .layer(ConcurrencyLimitLayer::new(256))
        .with_state(state)
}

