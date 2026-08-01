//! `Validator` trait — configuration validation contract.

use crate::api::types::config::LoadbalancerConfig;

/// Validates a [`LoadbalancerConfig`] before the pool is constructed.
///
/// Returns `Ok(())` when the config is valid, or an `Err` with a
/// human-readable description of the first violation found.
pub trait Validator {
    /// Validate the given loadbalancer configuration.
    fn validate(config: &LoadbalancerConfig) -> Result<(), String>;
}
