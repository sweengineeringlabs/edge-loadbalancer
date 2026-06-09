//! SAF public-API integration tests — covers all standalone `pub fn` in saf/loadbalancer_svc.rs.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, BackendId, LoadbalancerConfig, Outcome, Strategy,
    build_backend_pool, pool_backend_count, report_backend_outcome, select_backend,
    validate_loadbalancer_config,
};

fn two_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    }
}

// ---------------------------------------------------------------------------
// build_backend_pool
// ---------------------------------------------------------------------------

#[test]
fn test_build_backend_pool_fn_valid_config_returns_pool() {
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    // Verify pool is functional by selecting from it.
    select_backend(&pool).expect("pool must be selectable");
}

#[test]
fn test_build_backend_pool_fn_empty_backends_returns_invalid_config_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = build_backend_pool(config).unwrap_err();
    assert!(
        matches!(err, swe_edge_loadbalancer::LoadbalancerError::InvalidConfig(_)),
        "empty backends must be InvalidConfig: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// validate_loadbalancer_config
// ---------------------------------------------------------------------------

#[test]
fn test_validate_loadbalancer_config_fn_valid_returns_ok() {
    assert!(validate_loadbalancer_config(&two_backend_config()).is_ok());
}

#[test]
fn test_validate_loadbalancer_config_fn_zero_weight_returns_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 0,
        }],
    };
    assert!(
        validate_loadbalancer_config(&config).is_err(),
        "zero weight must fail validation"
    );
}

// ---------------------------------------------------------------------------
// select_backend
// ---------------------------------------------------------------------------

#[test]
fn test_select_backend_fn_returns_backend_from_healthy_pool() {
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    let backend = select_backend(&pool).expect("must select from healthy pool");
    assert!(!backend.url.is_empty());
}

#[test]
fn test_select_backend_fn_returns_error_when_all_backends_degraded() {
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    report_backend_outcome(&pool, &BackendId::new("https://api-1.internal"), Outcome::CircuitOpen);
    report_backend_outcome(&pool, &BackendId::new("https://api-2.internal"), Outcome::CircuitOpen);
    let err = select_backend(&pool).unwrap_err();
    assert!(matches!(
        err,
        swe_edge_loadbalancer::LoadbalancerError::NoHealthyBackends
    ));
}

// ---------------------------------------------------------------------------
// pool_backend_count
// ---------------------------------------------------------------------------

#[test]
fn test_pool_backend_count_fn_returns_configured_backend_count() {
    // @covers: pool_backend_count
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    assert_eq!(pool_backend_count(&pool), 2, "pool must report correct backend count");
}

#[test]
fn test_pool_backend_count_fn_single_backend_returns_one() {
    // @covers: pool_backend_count
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://sole.internal".to_string(),
            weight: 1,
        }],
    };
    let pool = build_backend_pool(config).expect("pool must build");
    assert_eq!(pool_backend_count(&pool), 1, "single-backend pool must report count of 1");
}

// ---------------------------------------------------------------------------
// report_backend_outcome
// ---------------------------------------------------------------------------

#[test]
fn test_report_backend_outcome_fn_success_keeps_backend_selectable() {
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    let id = BackendId::new("https://api-1.internal");
    report_backend_outcome(&pool, &id, Outcome::Success);
    // Pool must still be selectable.
    select_backend(&pool).expect("pool must remain selectable after Success outcome");
}

#[test]
fn test_report_backend_outcome_fn_failure_degrades_backend() {
    let pool = build_backend_pool(two_backend_config()).expect("pool must build");
    let id1 = BackendId::new("https://api-1.internal");
    let id2 = BackendId::new("https://api-2.internal");
    report_backend_outcome(&pool, &id1, Outcome::Failure { reason: "timeout".to_string() });
    report_backend_outcome(&pool, &id2, Outcome::Failure { reason: "timeout".to_string() });
    select_backend(&pool).unwrap_err();
}
