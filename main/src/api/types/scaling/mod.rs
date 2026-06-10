//! Scaling signal and decision types (ADR-013 feedback loop).

pub(crate) mod pool_snapshot;
pub(crate) mod scale_out_hint;
pub(crate) mod scaling_decision;

pub use pool_snapshot::PoolSnapshot;
pub use scale_out_hint::ScaleOutHint;
pub use scaling_decision::ScalingDecision;
