//! Integration tests for `ApplicationConfigBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{ApplicationConfigBuilder, Strategy};

#[test]
fn test_application_config_builder_new_builds_default_config() {
    let config = ApplicationConfigBuilder::new().build().expect("default build must succeed");
    // Default config has no backends and RoundRobin strategy.
    assert_eq!(config.strategy, Strategy::RoundRobin);
    assert!(config.backends.is_empty());
}

#[test]
fn test_application_config_builder_with_toml_parses_strategy() {
    let config = ApplicationConfigBuilder::new()
        .with_toml(
            r#"
            [loadbalancer]
            strategy = "weighted"
            "#,
        )
        .expect("valid toml must parse");
    assert_eq!(config.strategy, Strategy::Weighted);
}

#[test]
fn test_application_config_builder_with_toml_parses_backends() {
    let config = ApplicationConfigBuilder::new()
        .with_toml(
            r#"
            [loadbalancer]
            [[loadbalancer.backends]]
            url = "https://api-1.internal"
            weight = 1
            "#,
        )
        .expect("valid toml must parse");
    assert_eq!(config.backends.len(), 1);
    assert_eq!(config.backends[0].url, "https://api-1.internal");
}

#[test]
fn test_application_config_builder_with_invalid_toml_returns_error() {
    let result = ApplicationConfigBuilder::new().with_toml("not valid toml [[[");
    assert!(result.is_err(), "malformed TOML must fail");
}
