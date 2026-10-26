//! `LoadbalancerConfig` — TOML config schema for the load balancer.

use serde::{Deserialize, Serialize};

use crate::api::error::EgressError;
use crate::api::types::config::backend_config::BackendConfig;
use crate::api::types::strategy::Strategy;

/// TOML config schema for the `[loadbalancer]` section.
///
/// # Examples
///
/// ```toml
/// [loadbalancer]
/// strategy = "round-robin"
///
/// [[loadbalancer.backends]]
/// url = "https://api-1.internal"
/// weight = 1
///
/// [[loadbalancer.backends]]
/// url = "https://api-2.internal"
/// weight = 2
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadbalancerConfig {
    /// Backend selection strategy.
    #[serde(default)]
    pub strategy: Strategy,
    /// List of upstream backends.
    #[serde(default)]
    pub backends: Vec<BackendConfig>,
}

impl LoadbalancerConfig {
    /// Configuration section name used by composition layers.
    pub const SECTION_NAME: &'static str = "loadbalancer";

    /// Description for the optional load-balancer configuration section.
    pub const DESCRIPTION: &'static str =
        "backend pool load balancing (round-robin/weighted/least-connections)";

    /// Owning team for the optional load-balancer configuration section.
    pub const OWNER: &'static str = "platform-team";

    /// Parse from TOML text that contains a `[loadbalancer]` table.
    ///
    /// # Errors
    ///
    /// Returns `EgressError::ParseFailed` if the TOML is malformed or
    /// the `[loadbalancer]` section is missing the required shape.
    pub fn from_toml(toml_text: &str) -> Result<Self, EgressError> {
        /// Intermediate wrapper matching the `[loadbalancer]` TOML table.
        #[derive(Deserialize)]
        struct Wrapper {
            loadbalancer: LoadbalancerConfig,
        }
        let w: Wrapper =
            toml::from_str(toml_text).map_err(|e| EgressError::ParseFailed(e.to_string()))?;
        Ok(w.loadbalancer)
    }
}
