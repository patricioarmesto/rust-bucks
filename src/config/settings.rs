use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub observability: ObservabilitySettings,
    pub database: DatabaseSettings,
    pub pools: HashMap<String, PoolConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ObservabilitySettings {
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub connection_timeout_secs: u64,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoolConfig {
    pub read_timeout_secs: u64,
    pub write_timeout_secs: u64,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
