//! `LoadbalancerError` — error type for load balancer operations.

use thiserror::Error;

/// Errors produced by load balancer operations.
#[derive(Debug, Error)]
pub enum LoadbalancerError {
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
