//! `AutoscaleError` — error type for autoscale operations.

use thiserror::Error;

/// Errors produced by autoscale operations.
///
/// Split out of the old shared `LoadbalancerError` per ADR-001 — this
/// subdomain owns exactly the variant its own inherent constructors
/// (`HandlerInstancePool::build`) can produce.
#[derive(Debug, Error)]
pub enum AutoscaleError {
    /// The provided configuration is invalid.
    #[error("invalid autoscale configuration: {0}")]
    InvalidConfig(String),
}
