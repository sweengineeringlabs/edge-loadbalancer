//! `BackendConfig` — TOML config entry for a single backend.

use serde::{Deserialize, Serialize};

/// TOML configuration for a single backend entry.
///
/// ```toml
/// [[loadbalancer.backends]]
/// url = "https://api-1.internal"
/// weight = 1
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    /// Base URL of the upstream backend.
    pub url: String,
    /// Relative weight for the `Weighted` strategy (defaults to 1).
    #[serde(default = "BackendConfig::default_weight")]
    pub weight: u32,
}

impl BackendConfig {
    /// Returns the default backend weight (1).
    pub fn default_weight() -> u32 {
        1
    }
}
