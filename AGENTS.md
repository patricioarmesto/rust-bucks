# AGENTS.md

## Purpose

This document defines the architectural and operational rules for all contributors working on this Rust REST service.

The primary goal is:

> Minimize async-related complexity, production failures, deadlocks, cancellation bugs, and operational instability.

This service intentionally constrains async usage to maintain:
- predictability
- maintainability
- observability
- resilience
- developer productivity

All contributors are expected to follow these guidelines.

---

# Core Principles

## 1. Async at the edges only

Async is infrastructure, not business logic.

### Async is allowed in:
- HTTP handlers
- database access
- external API clients
- queues/message brokers
- cache clients
- background workers

### Async should NOT exist in:
- domain logic
- validation
- business rules
- transformations
- pricing logic
- authorization decisions

---

## 2. Prefer synchronous domain services

Preferred architecture:

```text
HTTP Layer (async)
    ↓
Application Layer (mostly sync)
    ↓
Domain Layer (sync)
    ↓
Repository Interfaces
    ↓
Infrastructure Layer (async)
```

Business logic should remain synchronous whenever possible.

Example:

```rust
pub fn calculate_price(order: Order) -> Price
```

NOT:

```rust
pub async fn calculate_price(order: Order) -> Price
```

unless actual IO is required.

---

## 3. Tokio is the standard runtime

The service standardizes on Tokio.

Approved async ecosystem:
- tokio
- axum
- tower
- reqwest
- sqlx
- tracing

Do not introduce:
- async-std
- smol
- custom runtimes
- mixed executor models

---

# HTTP Layer

## 4. Use Axum + Tower

HTTP services should be implemented using:
- axum for routing
- tower for middleware

Middleware concerns belong in Tower layers:
- auth
- rate limiting
- timeouts
- retries
- tracing
- load shedding
- circuit breakers

Avoid implementing these concerns ad-hoc inside handlers.

---

## 5. Handlers should orchestrate, not contain business logic

Handlers should:
- validate inputs
- call services
- map errors
- return responses

Handlers should NOT:
- implement complex business rules
- contain large workflows
- manage concurrency manually

---

# Concurrency Rules

## 6. No unbounded task spawning

Unbounded `tokio::spawn` usage is forbidden inside request paths.

---

## 7. Prefer structured concurrency

Preferred patterns:
- `tokio::join!`
- `tokio::try_join!`
- `JoinSet`

Avoid detached background tasks.

---

## 8. All concurrency must be bounded

Every resource must have explicit concurrency limits.

This includes:
- database calls
- outbound HTTP calls
- queue publishing
- background workers

---

## 9. Use backpressure intentionally

The service should degrade gracefully under load.

Preferred failure mode:
- reject excess work
- return 429 or 503

Forbidden failure mode:
- unbounded queue growth
- executor starvation
- cascading timeouts

---

# Async Safety Rules

## 10. Never hold locks across await points

Locks must be released before `.await`.

---

## 11. Avoid async mutexes when possible

Prefer:
- immutable data
- ownership transfer
- channels
- actor-like isolation

Avoid:
- shared mutable state
- nested locks
- global async mutexes

---

## 12. Prefer channels over shared state

Preferred architecture:

```text
Producer
    ↓
Bounded Channel
    ↓
Single Owner Worker
```

Use:
- mpsc
- broadcast
- watch

All channels must be bounded unless explicitly justified.

---

## 13. Unbounded channels are forbidden

Use bounded channels everywhere.

---

# Timeouts and Cancellation

## 14. Every async operation requires a timeout

Required for:
- database queries
- external HTTP calls
- queue operations
- lock acquisition
- background jobs

No indefinite awaits.

---

## 15. Cancellation must be considered explicitly

Dropping a future cancels it.

All workflows involving side effects must be:
- idempotent
- retry-safe
- cancellation-aware

Recommended patterns:
- idempotency keys
- transactional outbox
- saga orchestration

---

# Background Work

## 16. Separate request path from long-running workflows

Preferred architecture:

```text
HTTP Request
    ↓
Persist Command
    ↓
Enqueue Job
    ↓
Background Worker
```

---

## 17. Background tasks must support graceful shutdown

All workers must:
- receive shutdown signals
- stop accepting new work
- drain in-flight work
- flush telemetry

---

# Database Rules

## 18. Database pool sizes must be explicit

Do not rely on defaults.

---

## 19. Database transactions should be short-lived

Transactions must:
- avoid network calls
- avoid long computations
- avoid external dependencies

---

# Blocking Work

## 20. Never block Tokio worker threads

Forbidden inside async contexts:
- CPU-heavy computations
- blocking filesystem operations
- compression
- synchronous crypto
- large JSON serialization

Use:
- `spawn_blocking`
- dedicated worker pools

---

# Observability

## 21. Tracing is mandatory

Use:
- tracing
- tracing-subscriber
- OpenTelemetry

Every request should include:
- request_id
- trace_id
- span hierarchy

---

## 22. Log structure matters

Prefer structured logs.

Required fields:
- request_id
- operation
- duration
- outcome

---

# Error Handling

## 23. Use typed errors

Prefer:
- thiserror
- domain-specific error enums

Avoid:
- String errors
- anyhow in domain boundaries

---

## 24. Errors must carry operational context

Errors should contain:
- service name
- operation
- timeout details
- retry context

---

# Dependency Rules

## 25. Prefer mature ecosystem crates

Avoid experimental async dependencies unless strongly justified.

---

# Testing

## 26. Concurrency-sensitive code requires stress testing

Required for:
- worker pools
- retry systems
- queue processing
- cancellation paths
- timeout handling

---

## 27. Avoid timing-sensitive tests

Do not rely on arbitrary sleeps.

Prefer:
- deterministic synchronization
- barriers
- channels
- simulated clocks

---

# Architecture Preferences

## 28. Prefer actor-like ownership boundaries

Preferred:

```text
Component owns its state
    ↓
Communication via messages
```

Avoid:
- globally shared mutable state
- cross-component locking

---

## 29. Keep async signatures local

Avoid propagating async unnecessarily through the codebase.

---

## 30. Simplicity beats abstraction

Avoid premature abstraction:
- unnecessary traits
- excessive generics
- deeply nested async combinators

Prefer:
- explicit flows
- concrete types
- readable orchestration

---

# Code Review Checklist

Every PR should verify:

- [ ] No locks held across await points
- [ ] All external IO has timeouts
- [ ] Concurrency is bounded
- [ ] No untracked spawned tasks
- [ ] Business logic remains synchronous
- [ ] Channels are bounded
- [ ] Cancellation behavior considered
- [ ] Structured tracing added
- [ ] Errors contain operational context
- [ ] No blocking operations inside async runtime

---

# Guiding Philosophy

This service optimizes for:
- operational predictability
- resilience under load
- maintainability
- developer ergonomics
- correctness

We intentionally trade some raw throughput and abstraction flexibility for stability and simplicity.

Keep async contained.
Keep ownership explicit.
Keep concurrency bounded.
