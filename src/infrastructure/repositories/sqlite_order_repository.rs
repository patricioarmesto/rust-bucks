// ── Infrastructure: Repository Adapter ────────────────────────────────────
//
//  This file implements the domain's OrderRepository trait using SQLite.
//  Key architectural points:
//
//  • The trait is SYNC (defined in domain), but sqlx is ASYNC.
//    We bridge via tokio::runtime::Handle::block_on. This is the approved
//    pattern when a sync boundary must call async infrastructure.
//
//  • No async_trait needed — the impl is sync. The async sqlx calls are
//    hidden inside block_on.
//
//  • Anyhow::Result is fine here (infrastructure layer). The domain error
//    (OrderRepositoryError) is returned to callers.
//
//  • The domain knows NOTHING about sqlx, SQLite, or connection pools.
//    This adapter is the only place these concerns appear.
// ──────────────────────────────────────────────────────────────────────────

use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::runtime::Handle;
use uuid::Uuid;

use crate::domain::order::entity::Order;
use crate::domain::order::repository::{OrderRepository, OrderRepositoryError};
use crate::domain::order::state::OrderState;

// SqliteOrderRepository wraps a shared connection pool.
// Arc is used so the pool can be cloned cheaply (e.g. into Axum state).
// The pool itself is already Send + Sync — Arc just enables shared ownership.
#[derive(Clone)]
pub struct SqliteOrderRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteOrderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

impl OrderRepository for SqliteOrderRepository {
    // ── save ──────────────────────────────────────────────────────────────
    //  The trait says fn, not async fn. Inside we use Handle::block_on to
    //  run the async sqlx query synchronously.
    //
    //  This is SAFE as long as:
    //    1. We're called from a thread with a tokio runtime (Axum handler).
    //    2. The inner future completes quickly (just a DB roundtrip).
    //
    //  The domain error OrderRepositoryError::Persistence wraps whatever
    //  sqlx returns, keeping sqlx types out of the domain boundary.
    // ──────────────────────────────────────────────────────────────────────
    fn save(&self, order: &Order) -> Result<(), OrderRepositoryError> {
        Handle::current()
            .block_on(async {
                sqlx::query(
                    // Use INSERT OR REPLACE for idempotency — calling save
                    // twice with the same id updates rather than errors.
                    r#"
                    INSERT OR REPLACE INTO orders (
                        id, customer_name, drink, state, created_at, updated_at
                    )
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#,
                )
                // Domain values are serialized to strings for storage.
                // The reverse happens in find_by_id below.
                .bind(order.id.to_string())
                .bind(&order.customer_name)
                .bind(&order.drink)
                .bind(format!("{:?}", order.state))
                .bind(order.created_at.to_rfc3339())
                .bind(order.updated_at.to_rfc3339())
                .execute(&*self.pool)
                .await
            })
            .map_err(|e| OrderRepositoryError::Persistence(e.to_string()))?;

        Ok(())
    }

    // ── find_by_id ────────────────────────────────────────────────────────
    //  Same sync-to-async bridge pattern. Uses query_as with a tuple destructure
    //  because we don't have compile-time sqlx-data verification set up.
    //  (query! macro would need a DATABASE_URL at build time.)
    // ──────────────────────────────────────────────────────────────────────
    fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, OrderRepositoryError> {
        let id_str = id.to_string();

        let row = Handle::current()
            .block_on(async {
                sqlx::query_as::<_, (String, String, String, String, String, String)>(
                    r#"
                    SELECT id, customer_name, drink, state, created_at, updated_at
                    FROM orders
                    WHERE id = ?
                    "#,
                )
                .bind(&id_str)
                .fetch_optional(&*self.pool)
                .await
            })
            .map_err(|e| OrderRepositoryError::Persistence(e.to_string()))?;

        // Pattern match on Option<Row>. If None, return Ok(None) — no error,
        // the order simply doesn't exist. Domain handles "not found" semantics.
        let Some((id, customer_name, drink, state_str, created_at, updated_at)) = row else {
            return Ok(None);
        };

        // Deserialize the stored Debug string back into the domain enum.
        // A production system might prefer serde serialization for robustness,
        // but this minimal approach keeps the example focused.
        // Note: "Cancelled" is the British spelling from Rust's Debug output
        // of OrderState::Cancelled — we match it exactly.
        let state = match state_str.as_str() {
            "Created" => OrderState::Created,
            "Preparing" => OrderState::Preparing,
            "Ready" => OrderState::Ready,
            "Cancelled" => OrderState::Cancelled,
            _ => return Err(OrderRepositoryError::Persistence("invalid order state".into())),
        };

        // Every parse error is wrapped in OrderRepositoryError::Persistence,
        // which the application layer can log or map to an HTTP 500.
        // The domain never sees raw sqlx or parse errors.
        let order = Order {
            id: Uuid::parse_str(&id).map_err(|e| OrderRepositoryError::Persistence(e.to_string()))?,
            customer_name,
            drink,
            state,
            created_at: created_at
                .parse()
                .map_err(|e| OrderRepositoryError::Persistence(e.to_string()))?,
            updated_at: updated_at
                .parse()
                .map_err(|e| OrderRepositoryError::Persistence(e.to_string()))?,
        };

        Ok(Some(order))
    }
}
