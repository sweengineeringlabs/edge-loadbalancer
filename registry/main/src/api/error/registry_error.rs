//! `RegistryError` — error type for registry operations.

use thiserror::Error;

/// Errors produced by registry operations.
///
/// Split out of the old shared `LoadbalancerError` per ADR-001 — this
/// subdomain owns exactly the variant its own inherent constructors
/// (`TomlTenantRegistry::build`) can produce.
#[derive(Debug, Error)]
pub enum RegistryError {
    /// TOML deserialisation failed.
    #[error("failed to parse loadbalancer config: {0}")]
    ParseFailed(String),
}
