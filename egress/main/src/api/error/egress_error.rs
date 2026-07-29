//! `EgressError` — error type for egress backend-pool operations.

use thiserror::Error;

/// Errors produced by egress backend-pool operations.
///
/// Split out of the former shared `LoadbalancerError` per ADR-001's Revision
/// note: this crate's `BackendPool`/`BackendPoolInstance` are the sole real
/// signature owner of all three variants.
#[derive(Debug, Error)]
pub enum EgressError {
    /// No healthy backend is available for selection.
    ///
    /// All backends in the pool are `Degraded` or `Dead`.
    #[error("no healthy backends available in the pool")]
    NoHealthyBackends,

    /// The provided configuration is invalid.
    #[error("invalid loadbalancer configuration: {0}")]
    InvalidConfig(String),

    /// TOML deserialisation failed.
    #[error("failed to parse loadbalancer config: {0}")]
    ParseFailed(String),
}
