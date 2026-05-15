mod app;
mod config;
mod http;
mod observability;
mod shutdown;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = config::settings::Settings::load()?;

    observability::tracing::init(&settings)?;

    info!("starting rust-bucks");

    let app = app::build_app();

    let address = format!("{}:{}", settings.server.host, settings.server.port);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    info!(address = %address, "http server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal::shutdown_signal())
        .await?;

    info!("shutdown complete");

    Ok(())
}
