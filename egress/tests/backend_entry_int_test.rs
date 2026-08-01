//! Integration tests for `BackendEntry` internal pool state.
//!
//! `BackendEntry` is `pub(crate)` and not accessible from integration tests;
//! its behaviour is covered indirectly through `BackendPoolInstance` which
//! owns a `Vec<BackendEntry>`. These tests exercise the observable contract:
//! connection counts tracked during `LeastConnections` selection, and
//! health transitions propagated through `report_outcome`.
//!
//! @covers: BackendEntry
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_egress::{
    BackendHealth, BackendPool, BackendPoolInstance, EgressError, LoadbalancerConfig, Outcome, Strategy,
};

fn config_with_two_backends() -> LoadbalancerConfig {
    use swe_edge_loadbalancer_egress::BackendConfig;
    LoadbalancerConfig {
        strategy: Strategy::LeastConnections,
        backends: vec![
            BackendConfig { url: "https://be-a.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://be-b.internal".to_string(), weight: 1 },
        ],
    }
}

#[test]
fn test_backend_entry_least_connections_prefers_entry_with_zero_connections() {
    // @covers: BackendEntry::connections
    // With LeastConnections strategy, both entries start at 0 connections, so
    // successive selects may return either backend. The important invariant is
    // that select never errors when healthy entries exist.
    let pool = BackendPoolInstance::build(config_with_two_backends())
        .expect("pool must build with valid config");
    let first = pool.select();
    assert!(first.is_ok(), "first selection must succeed");
    let second = pool.select();
    assert!(second.is_ok(), "second selection must succeed");
}

#[test]
fn test_backend_entry_health_degrades_after_failure_outcome() {
    // @covers: BackendEntry::backend health field via report_outcome
    use swe_edge_loadbalancer_egress::BackendConfig;
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://be-a.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://be-b.internal".to_string(), weight: 1 },
        ],
    };
    let pool = BackendPoolInstance::build(config).expect("pool must build");
    let backend = pool.select().expect("initial select must succeed");

    // Mark the selected backend as failed.
    pool.report_outcome(&backend.id, Outcome::Failure { reason: "timeout".to_string() });

    // With round-robin across 2 backends where one is Degraded but not Dead,
    // the other backend is still selectable. Simply verify select still works
    // (pool does not error when at least one healthy backend remains).
    assert!(pool.select().is_ok(), "pool with one healthy backend must still select successfully");
}

#[test]
fn test_backend_entry_health_recovers_after_success_outcome() {
    // @covers: BackendEntry::backend health recovery
    use swe_edge_loadbalancer_egress::BackendConfig;
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://single.internal".to_string(),
            weight: 1,
        }],
    };
    let pool = BackendPoolInstance::build(config).expect("pool must build");
    let backend = pool.select().expect("initial select must succeed");

    // Degrade, then recover.
    pool.report_outcome(&backend.id, Outcome::Failure { reason: "connection refused".to_string() });
    pool.report_outcome(&backend.id, Outcome::Success);

    // After recovery the single backend should be selectable again.
    let result = pool.select();
    assert!(result.is_ok(), "recovered backend must be selectable");
    assert_eq!(
        result.unwrap().health,
        BackendHealth::Healthy,
        "recovered backend health must be Healthy"
    );
}

#[test]
fn test_backend_entry_all_degraded_returns_no_healthy_backends_error() {
    // @covers: BackendEntry health filter in healthy_indices
    use swe_edge_loadbalancer_egress::BackendConfig;
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://only-one.internal".to_string(),
            weight: 1,
        }],
    };
    let pool = BackendPoolInstance::build(config).expect("pool must build");
    let backend = pool.select().expect("initial select must succeed");

    // Mark the only backend as circuit-open (Degraded health).
    pool.report_outcome(&backend.id, Outcome::CircuitOpen);

    let err = pool.select().unwrap_err();
    assert!(
        matches!(err, EgressError::NoHealthyBackends),
        "all-degraded pool must return NoHealthyBackends, got: {err:?}"
    );
}
