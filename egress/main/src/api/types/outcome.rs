//! `Outcome` — result of a request sent to a backend.

use serde::{Deserialize, Serialize};

/// The result of a request dispatched to a backend.
///
/// Pass this to `BackendPool::report_outcome` so the pool can update
/// the backend's health state.
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
