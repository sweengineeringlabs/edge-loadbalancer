//! Integration tests for `LoadbalancerSvc` factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, BackendId, LoadbalancerConfig, LoadbalancerSvc, Outcome, Strategy,
};

fn two_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://svc-1.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://svc-2.internal".to_string(), weight: 1 },
        ],
    }
}

#[test]
fn test_loadbalancer_svc_struct_build_pool_returns_pool() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("pool must build");
    let backend = LoadbalancerSvc::select(&pool).expect("must select from healthy pool");
    assert!(!backend.url.is_empty(), "selected backend url must not be empty");
}

#[test]
fn test_loadbalancer_svc_struct_validate_config_valid_returns_ok() {
    let result = LoadbalancerSvc::validate_config(&two_backend_config());
    assert!(result.is_ok(), "valid config must pass: {result:?}");
}

#[test]
fn test_loadbalancer_svc_struct_validate_config_empty_backends_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(LoadbalancerSvc::validate_config(&config).is_err());
}

#[test]
fn test_loadbalancer_svc_struct_select_alternates_backends_round_robin() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("pool must build");
    let b1 = LoadbalancerSvc::select(&pool).expect("select 1");
    let b2 = LoadbalancerSvc::select(&pool).expect("select 2");
    assert_ne!(b1.url, b2.url, "round-robin must alternate");
}

#[test]
fn test_loadbalancer_svc_struct_report_outcome_degrades_backend() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("pool must build");
    let id1 = BackendId::new("https://svc-1.internal");
    let id2 = BackendId::new("https://svc-2.internal");
    // Degrade both
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::CircuitOpen);
    LoadbalancerSvc::report_outcome(&pool, &id2, Outcome::CircuitOpen);
    let err = LoadbalancerSvc::select(&pool).unwrap_err();
    assert!(
        matches!(
            err,
            swe_edge_loadbalancer::LoadbalancerError::Egress(
                swe_edge_loadbalancer::EgressError::NoHealthyBackends
            )
        ),
        "all degraded must return NoHealthyBackends: {err:?}"
    );
}

#[test]
fn test_loadbalancer_svc_struct_report_outcome_success_restores_backend() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("pool must build");
    let id1 = BackendId::new("https://svc-1.internal");
    let id2 = BackendId::new("https://svc-2.internal");
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::Failure { reason: "err".to_string() });
    LoadbalancerSvc::report_outcome(&pool, &id2, Outcome::Failure { reason: "err".to_string() });
    // Restore one
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::Success);
    let b = LoadbalancerSvc::select(&pool).expect("restored backend must be selectable");
    assert_eq!(b.url, "https://svc-1.internal");
}
