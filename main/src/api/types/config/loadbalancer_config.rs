//! `LoadbalancerConfig` — TOML config schema for the load balancer.

use serde::{Deserialize, Serialize};

use crate::api::error::LoadbalancerError;
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

impl swe_edge_configbuilder::ConfigSection for LoadbalancerConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — required trait method with constant return
        "loadbalancer"
    }
}

impl swe_edge_configbuilder::OptionalSection for LoadbalancerConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — required trait method with constant return
        "loadbalancer"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "backend pool load balancing (round-robin/weighted/least-connections)",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl LoadbalancerConfig {
    /// Parse from TOML text that contains a `[loadbalancer]` table.
    ///
    /// # Errors
    ///
    /// Returns `LoadbalancerError::ParseFailed` if the TOML is malformed or
    /// the `[loadbalancer]` section is missing the required shape.
    pub fn from_toml(toml_text: &str) -> Result<Self, LoadbalancerError> {
        /// Intermediate wrapper matching the `[loadbalancer]` TOML table.
        #[derive(Deserialize)]
        struct Wrapper {
            loadbalancer: LoadbalancerConfig,
        }
        let w: Wrapper = toml::from_str(toml_text)
            .map_err(|e| LoadbalancerError::ParseFailed(e.to_string()))?;
        Ok(w.loadbalancer)
    }
}
