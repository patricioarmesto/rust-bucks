// ── Domain State ──────────────────────────────────────────────────────────
//
//  Serde derives are acceptable here because this enum is used across the
//  service boundary (e.g. serialized in API responses or persisted).
//
//  However: serialization format decisions (JSON, etc.) live in the HTTP
//  layer, not here. The domain only provides the derives so infrastructure
//  can do its job — it never calls serde_json::to_string itself.
// ──────────────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderState {
    Created,
    Preparing,
    Ready,
    Cancelled,
}
