use axum::{Router, routing::get, routing::post};

use crate::http::{
    handlers::{health, metrics, orders::create, orders::get, ping},
    state::AppState,
};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/metrics", get(metrics::metrics))
        .route("/ping", get(ping::ping))
        .route("/orders", post(create::create_order))
        .route("/orders/:order_id", get(get::get_order))
        .with_state(state)
}
