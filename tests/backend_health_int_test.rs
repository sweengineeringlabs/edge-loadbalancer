//! Integration tests for `BackendHealth` state transitions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, BackendHealth, BackendId, LoadbalancerConfig, Outcome, Strategy,
    build_backend_pool, report_backend_outcome, select_backend,
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
fn test_backend_health_initial_state_is_healthy() {
    let pool = build_backend_pool(single_backend_config()).expect("pool must build");
    let backend = select_backend(&pool).expect("must select a healthy backend");
    assert_eq!(backend.health, BackendHealth::Healthy);
}

#[test]
fn test_backend_health_failure_outcome_transitions_to_degraded() {
    let pool = build_backend_pool(single_backend_config()).expect("pool must build");
    let id = BackendId::new("https://api-1.internal");
    report_backend_outcome(&pool, &id, Outcome::Failure { reason: "timeout".to_string() });
    // After degradation all backends are degraded — select must fail
    let err = select_backend(&pool).unwrap_err();
    assert!(
        matches!(err, swe_edge_loadbalancer::LoadbalancerError::NoHealthyBackends),
        "expected NoHealthyBackends after failure, got {err:?}"
    );
}

#[test]
fn test_backend_health_circuit_open_transitions_to_degraded() {
    let pool = build_backend_pool(single_backend_config()).expect("pool must build");
    let id = BackendId::new("https://api-1.internal");
    report_backend_outcome(&pool, &id, Outcome::CircuitOpen);
    let err = select_backend(&pool).unwrap_err();
    assert!(matches!(
        err,
        swe_edge_loadbalancer::LoadbalancerError::NoHealthyBackends
    ));
}

#[test]
fn test_backend_health_success_outcome_restores_to_healthy() {
    let pool = build_backend_pool(single_backend_config()).expect("pool must build");
    let id = BackendId::new("https://api-1.internal");
    // Degrade first
    report_backend_outcome(&pool, &id, Outcome::Failure { reason: "err".to_string() });
    // Restore
    report_backend_outcome(&pool, &id, Outcome::Success);
    // Should be selectable again
    select_backend(&pool).expect("backend must be healthy again after Success outcome");
}
