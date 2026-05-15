use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{errors::OrderError, state::OrderState};

// ── Domain Entity ─────────────────────────────────────────────────────────
//
//  This is the core domain layer. Every decision here follows strict rules:
//
//  • NO async — no .await, no async fn, no tokio. Domain logic is pure
//    synchronous transformation of in-memory state. Async is infrastructure.
//
//  • NO database access — no queries, no repositories, no connection pools.
//    Persistence is handled by the infrastructure layer one level up.
//
//  • NO tracing — no log statements, no span instrumentation. The domain
//    doesn't know about observability. Callers add context if needed.
//
//  • NO runtime dependencies — no axum, no tokio, no tower. Only pure Rust
//    and domain-specific crates (uuid, chrono for value types).
//
//  • NO infrastructure concerns — no HTTP status codes, no serialization
//    formats, no middleware, no request/response types.
//
//  What IS allowed:
//    - Pure functions and state machines
//    - Domain errors via thiserror (typed, exhaustive)
//    - Immutable data and controlled mutation via &mut self
//    - Value objects (Uuid, DateTime, custom newtypes)
//
//  Think of this file as a state machine that encodes business rules.
//  Every method documents a legal state transition, nothing more.
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub customer_name: String,
    pub drink: String,
    pub state: OrderState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn create(customer_name: String, drink: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(), // requires uuid feature "v4"
            customer_name,
            drink,
            state: OrderState::Created,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_preparing(&mut self) -> Result<(), OrderError> {
        if self.state == OrderState::Cancelled {
            return Err(OrderError::InvalidStateTransition);
        }

        self.state = OrderState::Preparing;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn mark_ready(&mut self) -> Result<(), OrderError> {
        if self.state != OrderState::Preparing {
            return Err(OrderError::InvalidStateTransition);
        }

        self.state = OrderState::Ready;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), OrderError> {
        if self.state == OrderState::Cancelled {
            return Err(OrderError::AlreadyCancelled);
        }

        self.state = OrderState::Cancelled;
        self.updated_at = Utc::now();

        Ok(())
    }
}
