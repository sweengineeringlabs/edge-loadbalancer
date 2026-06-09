//! `ApplicationConfigBuilder` — fluent builder for application-level loadbalancer config.
//!
//! Maps to `config/application.toml`. Consumers use this builder to apply
//! application-level overrides on top of the crate's TOML defaults.

use crate::api::error::LoadbalancerError;
use crate::api::types::config::LoadbalancerConfig;

/// Fluent builder for application-level loadbalancer configuration.
///
/// Corresponds to `config/application.toml`.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer::ApplicationConfigBuilder;
///
/// let config = ApplicationConfigBuilder::new()
///     .with_toml(r#"
///         [loadbalancer]
///         strategy = "round-robin"
///     "#)
///     .unwrap();
/// assert!(config.backends.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct ApplicationConfigBuilder {
    toml_override: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Create a new builder with no overrides applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a TOML string override. The string must contain a `[loadbalancer]` section.
    pub fn with_toml(mut self, toml: impl Into<String>) -> Result<LoadbalancerConfig, LoadbalancerError> {
        self.toml_override = Some(toml.into());
        self.build()
    }

    /// Build a [`LoadbalancerConfig`] from the applied overrides.
    ///
    /// Returns a default (empty backends, round-robin strategy) when no overrides
    /// have been set.
    ///
    /// # Errors
    ///
    /// Returns `LoadbalancerError::ParseFailed` if the TOML is malformed.
    pub fn build(self) -> Result<LoadbalancerConfig, LoadbalancerError> {
        match self.toml_override {
            Some(toml) => LoadbalancerConfig::from_toml(&toml),
            None => Ok(LoadbalancerConfig::default()),
        }
    }
}
