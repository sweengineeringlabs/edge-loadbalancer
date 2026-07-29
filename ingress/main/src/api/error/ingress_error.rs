//! `IngressError` — error type for ingress load balancer operations.

use thiserror::Error;

/// Errors produced by ingress load balancer operations (ADR-012).
///
/// Split out of the former shared `LoadbalancerError` per ADR-001: this
/// subdomain owns exactly the variant its trait contract actually returns.
#[derive(Debug, Error)]
pub enum IngressError {
    /// The provided node/configuration is invalid.
    #[error("invalid ingress loadbalancer configuration: {0}")]
    InvalidConfig(String),
}
