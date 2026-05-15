mod app;
mod application;
mod config;
mod domain;
mod http;
mod infrastructure;
mod observability;
mod shutdown;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use tracing::info;

use crate::domain::order::repository::OrderRepository;
use crate::http::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = config::settings::Settings::load()?;

    observability::tracing::init(&settings)?;

    let pool = infrastructure::db::sqlite::create_pool(&settings.database).await?;

    let pool_config = settings
        .pools
        .get("orders")
        .context("missing [pools.orders] configuration")?;

    let order_repository: Arc<dyn OrderRepository> = Arc::new(
        infrastructure::repositories::sqlite_order_repository::SqliteOrderRepository::new(
            pool,
            Duration::from_secs(pool_config.read_timeout_secs),
            Duration::from_secs(pool_config.write_timeout_secs),
        ),
    );

    let state = AppState {
        create_order: Arc::new(application::orders::create::CreateOrderUseCase::new(
            Arc::clone(&order_repository),
        )),
        get_order: Arc::new(application::orders::get::GetOrderUseCase::new(Arc::clone(
            &order_repository,
        ))),
        cancel_order: Arc::new(application::orders::cancel::CancelOrderUseCase::new(
            Arc::clone(&order_repository),
        )),
    };

    info!("starting rust-bucks");

    let app = app::build_app(state);

    let address = format!("{}:{}", settings.server.host, settings.server.port);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    info!(address = %address, "http server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal::shutdown_signal())
        .await?;

    info!("shutdown complete");

    Ok(())
}
