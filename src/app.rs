use axum::Router;

use crate::http::routes;

pub fn build_app() -> Router {
    Router::new().merge(routes::routes())
}
