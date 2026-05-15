use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::settings::Settings;

pub fn init(settings: &Settings) -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_new(&settings.observability.log_level)?;

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    Ok(())
}
