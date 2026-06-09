//! Integration tests for `validate_loadbalancer_config` SAF wrapper.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{BackendConfig, LoadbalancerConfig, Strategy, validate_loadbalancer_config};

fn valid_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 1,
        }],
    }
}

#[test]
fn test_validate_loadbalancer_config_valid_config_returns_ok() {
    let result = validate_loadbalancer_config(&valid_config());
    assert!(result.is_ok(), "valid config must pass validation: {result:?}");
}

#[test]
fn test_validate_loadbalancer_config_empty_backends_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = validate_loadbalancer_config(&config).unwrap_err();
    assert!(
        err.contains("at least one backend"),
        "empty backends error must mention 'at least one backend': {err}"
    );
}

#[test]
fn test_validate_loadbalancer_config_empty_url_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig { url: String::new(), weight: 1 }],
    };
    let err = validate_loadbalancer_config(&config).unwrap_err();
    assert!(
        err.contains("url"),
        "empty URL error must mention 'url': {err}"
    );
}

#[test]
fn test_validate_loadbalancer_config_zero_weight_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 0,
        }],
    };
    let err = validate_loadbalancer_config(&config).unwrap_err();
    assert!(
        err.contains("weight"),
        "zero weight error must mention 'weight': {err}"
    );
}

#[test]
fn test_validate_loadbalancer_config_multiple_backends_all_valid_returns_ok() {
    let config = LoadbalancerConfig {
        strategy: Strategy::Weighted,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 3 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    };
    assert!(validate_loadbalancer_config(&config).is_ok());
}
