//! `BackendHealth` — health state of a backend in the pool.

use serde::{Deserialize, Serialize};

/// Observed health state of a single backend.
///
/// Transitions are driven by `BackendPool::report_outcome`:
/// - `Outcome::Success` → `Healthy`
/// - `Outcome::Failure | Outcome::CircuitOpen` → `Degraded`
///
/// `Dead` is a terminal state that can only be set by an operator
/// (not driven by `report_outcome`). Dead backends are never selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendHealth {
    /// Backend is responding normally and eligible for selection.
    Healthy,
    /// Backend has experienced recent failures. Eligible for
    /// selection but deprioritised by some strategies.
    Degraded,
    /// Backend is permanently out of service. Never selected.
    Dead,
}
