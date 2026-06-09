//! Integration tests for `BackendConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{BackendConfig, LoadbalancerConfig, Strategy};

fn parse_backends(toml: &str) -> Vec<BackendConfig> {
    LoadbalancerConfig::from_toml(toml).expect("valid toml").backends
}

#[test]
fn test_backend_config_struct_url_field_is_required_in_toml() {
    let backends = parse_backends(
        r#"
        [loadbalancer]
        [[loadbalancer.backends]]
        url = "https://api-1.internal"
        "#,
    );
    assert_eq!(backends[0].url, "https://api-1.internal");
}

#[test]
fn test_backend_config_struct_weight_defaults_to_one_when_absent() {
    let backends = parse_backends(
        r#"
        [loadbalancer]
        [[loadbalancer.backends]]
        url = "https://api-1.internal"
        "#,
    );
    assert_eq!(backends[0].weight, 1, "weight must default to 1 when absent");
}

#[test]
fn test_backend_config_struct_explicit_weight_overrides_default() {
    let backends = parse_backends(
        r#"
        [loadbalancer]
        [[loadbalancer.backends]]
        url = "https://api-1.internal"
        weight = 5
        "#,
    );
    assert_eq!(backends[0].weight, 5);
}

#[test]
fn test_backend_config_struct_default_weight_fn_returns_one() {
    assert_eq!(BackendConfig::default_weight(), 1);
}

#[test]
fn test_backend_config_struct_multiple_backends_parse_correctly() {
    let config = LoadbalancerConfig::from_toml(
        r#"
        [loadbalancer]
        strategy = "weighted"

        [[loadbalancer.backends]]
        url = "https://api-1.internal"
        weight = 3

        [[loadbalancer.backends]]
        url = "https://api-2.internal"
        weight = 1
        "#,
    )
    .expect("valid toml");
    assert_eq!(config.backends.len(), 2);
    assert_eq!(config.strategy, Strategy::Weighted);
    assert_eq!(config.backends[0].weight, 3);
    assert_eq!(config.backends[1].weight, 1);
}
