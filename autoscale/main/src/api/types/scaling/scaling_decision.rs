//! `ScalingDecision` — output of a scaling policy evaluation.

use crate::api::types::scaling::scale_out_hint::ScaleOutHint;

/// The action a scaling policy decided on after evaluating a
/// [`PoolSnapshot`](crate::api::types::scaling::PoolSnapshot).
///
/// In-process decisions (`ScaleUp` / `ScaleDown`) adjust a pool's concurrency
/// cap; cross-node decisions (`ScaleOut` / `ScaleIn`) add or remove nodes via
/// the ingress load balancer's infrastructure plug-in.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalingDecision {
    /// No action — the pool is within thresholds.
    Hold,
    /// Raise the concurrency cap by `n` slots.
    ScaleUp(usize),
    /// Lower the concurrency cap by `n` slots.
    ScaleDown(usize),
    /// Add a node using the given deployment hint.
    ScaleOut(ScaleOutHint),
    /// Remove a node.
    ScaleIn,
}
