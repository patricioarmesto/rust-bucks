use uuid::Uuid;

use super::entity::Order;

// ── Repository Port ───────────────────────────────────────────────────────
//
//  This trait is the "port" in Ports & Adapters (Hexagonal Architecture).
//  It lives in the domain to define the contract, but observe:
//
//  • NO async_trait — the interface is synchronous. Async is an
//    infrastructure implementation detail. The adapter bridges the gap
//    (e.g. via tokio::runtime::Handle::block_on or spawn_blocking).
//
//  • NO anyhow — return a domain-specific error type instead.
//    Callers in the application layer map this to infrastructure errors.
//
//  • The domain defines WHAT it needs, not HOW it's delivered.
// ──────────────────────────────────────────────────────────────────────────

pub trait OrderRepository: Send + Sync {
    fn save(&self, order: &Order) -> Result<(), OrderRepositoryError>;

    fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, OrderRepositoryError>;
}

// ── Repository Error ──────────────────────────────────────────────────────
//
//  Typed error for persistence failures. Keeps the domain boundary
//  free of infrastructure error types (sqlx, deadpool, etc.).
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum OrderRepositoryError {
    #[error("persistence error: {0}")]
    Persistence(String),
}
