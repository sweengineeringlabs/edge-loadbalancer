//! `ScalingSignal` trait — pool load snapshot producer.

use crate::api::types::scaling::PoolSnapshot;

/// Produces a point-in-time load snapshot for scaling policy evaluation.
///
/// Implemented by instance pools (in-process side, ADR-013) and backend
/// pools (egress side, ADR-011). A runtime-side sampler reads snapshots on a
/// fixed tick and feeds them to a scaling policy.
pub trait ScalingSignal: Send + Sync {
    /// Capture the pool's current load signal.
    fn snapshot(&self) -> PoolSnapshot;
}
