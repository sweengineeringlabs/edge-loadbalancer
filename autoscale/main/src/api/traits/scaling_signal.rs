//! `ScalingSignal` trait — pool load snapshot producer.

use crate::api::types::scaling::PoolSnapshot;

/// Produces a point-in-time load snapshot for scaling policy evaluation.
///
/// Implemented by instance pools (in-process side, ADR-013). A runtime-side
/// sampler reads snapshots on a fixed tick and feeds them to a scaling
/// policy.
///
/// Note: the egress-side backend pool (`swe-edge-loadbalancer-egress`,
/// ADR-011) does not implement this trait today, despite an earlier version
/// of this doc comment claiming otherwise — see ADR-001's "Relationship
/// guidance applied" section. If it ever does, `-egress` would need to
/// depend on this crate for the trait at that point.
pub trait ScalingSignal: Send + Sync {
    /// Capture the pool's current load signal.
    fn snapshot(&self) -> PoolSnapshot;
}
