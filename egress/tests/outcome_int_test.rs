//! Integration tests for `Outcome` variants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_egress::Outcome;

#[test]
fn test_outcome_success_variant_is_constructible() {
    let outcome = Outcome::Success;
    assert!(matches!(outcome, Outcome::Success));
}

#[test]
fn test_outcome_failure_variant_stores_reason() {
    let outcome = Outcome::Failure {
        reason: "connection refused".to_string(),
    };
    match outcome {
        Outcome::Failure { reason } => assert_eq!(reason, "connection refused"),
        other => panic!("expected Failure variant, got {other:?}"),
    }
}

#[test]
fn test_outcome_circuit_open_variant_is_constructible() {
    let outcome = Outcome::CircuitOpen;
    assert!(matches!(outcome, Outcome::CircuitOpen));
}

#[test]
fn test_outcome_failure_empty_reason_is_accepted() {
    // Empty reason is structurally valid — upstream layers may use a sentinel.
    let outcome = Outcome::Failure { reason: String::new() };
    assert!(matches!(outcome, Outcome::Failure { .. }));
}
