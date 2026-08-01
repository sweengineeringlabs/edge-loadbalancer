//! `Outcome` — result of a request dispatched to a handler pool.
//!
//! Structurally identical to (and copied from) `swe-edge-loadbalancer`'s
//! `Outcome`, used by both `BackendPool::report_outcome` (egress,
//! `swe-edge-loadbalancer-egress`) and `InstancePool::report_outcome`
//! (autoscale, this crate). ADR-001 assigns `Outcome` to the egress
//! subdomain by trait-signature count, but `InstancePool::report_outcome`
//! also names it directly — this crate needs its own copy to build
//! standalone against only `swe-edge-loadbalancer-tenant`, per this issue's
//! acceptance criteria, rather than pulling in the entire egress crate for
//! one enum. Flagged here so it isn't a surprise when `-egress` lands.

use serde::{Deserialize, Serialize};

/// The result of a request dispatched to a backend or handler.
///
/// Pass this to `InstancePool::report_outcome` so the pool can update its
/// error-rate tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    /// The request completed successfully.
    Success,
    /// The request failed with a reason description.
    Failure {
        /// Human-readable description of what went wrong.
        reason: String,
    },
    /// The circuit breaker for the backend is open; no attempt was made.
    CircuitOpen,
}
