use thiserror::Error;

// ── Domain Errors ─────────────────────────────────────────────────────────
//
//  Typed, exhaustive error enum — no String errors, no anyhow.
//  Every variant represents a business rule violation.
//
//  Callers (application layer) map these to HTTP status codes
//  or other output boundaries. The domain never references HTTP.
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("order is already cancelled")]
    AlreadyCancelled,

    #[error("invalid state transition")]
    InvalidStateTransition,
}
