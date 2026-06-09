//! `BackendPool` trait — primary load-balancer contract.

use crate::api::error::LoadbalancerError;
use crate::api::types::backend::Backend;
use crate::api::types::backend::BackendId;
use crate::api::types::outcome::Outcome;

/// A pool of backends from which one is selected per request.
///
/// Implementors maintain health state per backend and apply a
/// [`Strategy`](crate::api::types::strategy::Strategy) to select among
/// the healthy backends.
pub trait BackendPool: Send + Sync {
    /// Select a healthy backend from the pool.
    ///
    /// Returns `Err(LoadbalancerError::NoHealthyBackends)` when every backend
    /// is `Degraded` or `Dead`.
    fn select(&self) -> Result<Backend, LoadbalancerError>;

    /// Record the outcome of a request sent to the given backend.
    ///
    /// - `Outcome::Success` transitions the backend to `Healthy`.
    /// - `Outcome::Failure` or `Outcome::CircuitOpen` transitions it to `Degraded`.
    fn report_outcome(&self, id: &BackendId, outcome: Outcome);
}
