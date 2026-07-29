//! Integration tests for `BackendPoolInstance` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, BackendPoolInstance, LoadbalancerConfig, LoadbalancerSvc, Strategy,
};

fn single_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 1,
        }],
    }
}

#[test]
fn test_backend_pool_instance_struct_is_debug_printable() {
    let pool: BackendPoolInstance =
        LoadbalancerSvc::build_pool(single_backend_config()).expect("pool must build");
    let dbg = format!("{pool:?}");
    assert!(!dbg.is_empty(), "Debug impl must produce non-empty output");
    assert!(
        dbg.contains("BackendPoolInstance"),
        "Debug must include type name: {dbg}"
    );
}

#[test]
fn test_backend_pool_instance_struct_builds_from_valid_config() {
    let result = LoadbalancerSvc::build_pool(single_backend_config());
    assert!(
        result.is_ok(),
        "BackendPoolInstance must build from valid config"
    );
}

#[test]
fn test_backend_pool_instance_struct_build_fails_with_empty_backends() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = LoadbalancerSvc::build_pool(config).unwrap_err();
    assert!(
        matches!(err, swe_edge_loadbalancer::LoadbalancerError::InvalidConfig(_)),
        "empty backends must return InvalidConfig: {err:?}"
    );
}
