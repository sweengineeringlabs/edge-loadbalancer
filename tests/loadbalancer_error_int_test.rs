//! Integration tests for `LoadbalancerError`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::LoadbalancerError;

#[test]
fn test_loadbalancer_error_no_healthy_backends_has_message() {
    let err = LoadbalancerError::NoHealthyBackends;
    let msg = err.to_string();
    assert!(
        msg.contains("healthy") || msg.contains("backend"),
        "NoHealthyBackends error message must mention 'healthy' or 'backend': {msg}"
    );
}

#[test]
fn test_loadbalancer_error_invalid_config_includes_reason() {
    let err = LoadbalancerError::InvalidConfig("empty backend list".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("empty backend list"),
        "InvalidConfig error must include reason: {msg}"
    );
}

#[test]
fn test_loadbalancer_error_parse_failed_includes_reason() {
    let err = LoadbalancerError::ParseFailed("expected table".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("expected table"),
        "ParseFailed error must include reason: {msg}"
    );
}

#[test]
fn test_loadbalancer_error_variants_are_distinct() {
    let e1 = LoadbalancerError::NoHealthyBackends;
    let e2 = LoadbalancerError::InvalidConfig("x".to_string());
    let e3 = LoadbalancerError::ParseFailed("y".to_string());
    // All three have different Display outputs.
    assert_ne!(e1.to_string(), e2.to_string());
    assert_ne!(e1.to_string(), e3.to_string());
    assert_ne!(e2.to_string(), e3.to_string());
}

#[test]
fn test_loadbalancer_error_debug_is_implemented() {
    let err = LoadbalancerError::NoHealthyBackends;
    let dbg = format!("{err:?}");
    assert!(!dbg.is_empty(), "Debug impl must produce non-empty output");
}
