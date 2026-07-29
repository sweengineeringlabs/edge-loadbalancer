//! Integration tests for the `Validator` trait, implemented by `BackendPoolInstance`.
//!
//! Migrated from `swe-edge-loadbalancer`'s `tests/validator_int_test.rs` per
//! ADR-001. Construction changes from `LoadbalancerSvc::validate_config(&config)`
//! (the umbrella crate's facade, not present in this leaf crate) to
//! `BackendPoolInstance::validate(&config)` directly — the same
//! `impl Validator for BackendPoolInstance` in `core/pool/backend_pool_instance.rs`
//! the facade method delegated to. Assertions and covered behaviour are unchanged.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_egress::{BackendConfig, BackendPoolInstance, LoadbalancerConfig, Strategy, Validator};

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
fn test_validator_valid_config_returns_ok() {
    let result = BackendPoolInstance::validate(&valid_config());
    assert!(result.is_ok(), "valid config must pass validation: {result:?}");
}

#[test]
fn test_validator_empty_backends_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = BackendPoolInstance::validate(&config).unwrap_err();
    assert!(
        err.contains("at least one backend"),
        "empty backends error must mention 'at least one backend': {err}"
    );
}

#[test]
fn test_validator_empty_url_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig { url: String::new(), weight: 1 }],
    };
    let err = BackendPoolInstance::validate(&config).unwrap_err();
    assert!(
        err.contains("url"),
        "empty URL error must mention 'url': {err}"
    );
}

#[test]
fn test_validator_zero_weight_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 0,
        }],
    };
    let err = BackendPoolInstance::validate(&config).unwrap_err();
    assert!(
        err.contains("weight"),
        "zero weight error must mention 'weight': {err}"
    );
}

#[test]
fn test_validator_multiple_backends_all_valid_returns_ok() {
    let config = LoadbalancerConfig {
        strategy: Strategy::Weighted,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 3 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    };
    assert!(BackendPoolInstance::validate(&config).is_ok());
}
