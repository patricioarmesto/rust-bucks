// ── Infrastructure Layer ──────────────────────────────────────────────────
//
//  This is the opposite of the domain layer. Here, ALL of these are OK:
//
//  • async fn — database access is async (sqlx). The runtime handles this.
//  • anyhow::Result — application-level errors can be catch-all.
//  • tokio::time::timeout — mandatory per Rule 14: every async operation
//    requires a timeout. No indefinite awaits.
//  • explicit pool sizing — Rule 18: database pool sizes must be explicit.
//    Never rely on defaults.
//
//  The domain knows NOTHING about this file. It defines a sync trait
//  (OrderRepository); this file implements the async bridge.
//
//  Key infrastructure decisions:
//    - bounded pool (max_connections from config)
//    - connect timeout via tokio::time::timeout
//    - sqlx with sqlite backend (lightweight, no external server)
// ──────────────────────────────────────────────────────────────────────────

use std::time::Duration;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::time::timeout;

use crate::config::settings::DatabaseSettings;

pub async fn create_pool(config: &DatabaseSettings) -> anyhow::Result<SqlitePool> {
    let pool = timeout(
        Duration::from_secs(config.connection_timeout_secs),
        SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.url),
    )
    .await
    .map_err(|_| {
        anyhow::anyhow!(
            "database connection timed out after {}s",
            config.connection_timeout_secs
        )
    })??;

    Ok(pool)
}
