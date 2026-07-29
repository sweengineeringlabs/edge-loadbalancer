//! `Strategy` — backend selection algorithm.

use serde::{Deserialize, Serialize};

/// The algorithm used to select a backend from the pool.
///
/// Configured via `[loadbalancer]` TOML:
///
/// ```toml
/// [loadbalancer]
/// strategy = "round-robin"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Strategy {
    /// Distribute requests evenly across healthy backends in order.
    #[default]
    RoundRobin,
    /// Distribute requests proportionally to each backend's `weight`.
    Weighted,
    /// Send each request to the healthy backend with fewest active connections.
    LeastConnections,
}
