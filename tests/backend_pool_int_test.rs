//! Integration tests for `BackendPoolInstance` via the SAF functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, BackendId, LoadbalancerConfig, LoadbalancerSvc, Outcome, Strategy,
};

fn two_backend_rr_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    }
}

#[test]
fn test_backend_pool_round_robin_selects_backends_in_order() {
    let pool = LoadbalancerSvc::build_pool(two_backend_rr_config()).expect("pool must build");
    let first = LoadbalancerSvc::select(&pool).expect("first select must succeed");
    let second = LoadbalancerSvc::select(&pool).expect("second select must succeed");
    // Round-robin must cycle: first != second
    assert_ne!(
        first.url, second.url,
        "round-robin must alternate between backends on consecutive calls"
    );
}

#[test]
fn test_backend_pool_round_robin_cycles_back_to_first() {
    let pool = LoadbalancerSvc::build_pool(two_backend_rr_config()).expect("pool must build");
    let b0 = LoadbalancerSvc::select(&pool).expect("select 0");
    let b1 = LoadbalancerSvc::select(&pool).expect("select 1");
    let b2 = LoadbalancerSvc::select(&pool).expect("select 2");
    // The third pick must equal the first (2-backend pool cycles every 2 picks).
    assert_eq!(b0.url, b2.url, "round-robin must cycle: b0 == b2");
    assert_ne!(b0.url, b1.url, "round-robin must alternate: b0 != b1");
}

#[test]
fn test_backend_pool_degraded_backend_excluded_from_selection() {
    let pool = LoadbalancerSvc::build_pool(two_backend_rr_config()).expect("pool must build");
    let id1 = BackendId::new("https://api-1.internal");
    // Degrade the first backend
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::Failure { reason: "err".to_string() });
    // All subsequent selections must return only api-2
    for _ in 0..5 {
        let b = LoadbalancerSvc::select(&pool).expect("must select the healthy backend");
        assert_eq!(
            b.url, "https://api-2.internal",
            "degraded backend must never be selected"
        );
    }
}

#[test]
fn test_backend_pool_all_degraded_returns_no_healthy_backends_error() {
    let pool = LoadbalancerSvc::build_pool(two_backend_rr_config()).expect("pool must build");
    let id1 = BackendId::new("https://api-1.internal");
    let id2 = BackendId::new("https://api-2.internal");
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::CircuitOpen);
    LoadbalancerSvc::report_outcome(&pool, &id2, Outcome::CircuitOpen);
    let err = LoadbalancerSvc::select(&pool).unwrap_err();
    assert!(
        matches!(err, swe_edge_loadbalancer::LoadbalancerError::NoHealthyBackends),
        "all backends degraded must return NoHealthyBackends: {err:?}"
    );
}

#[test]
fn test_backend_pool_report_outcome_success_restores_healthy() {
    let pool = LoadbalancerSvc::build_pool(two_backend_rr_config()).expect("pool must build");
    let id1 = BackendId::new("https://api-1.internal");
    let id2 = BackendId::new("https://api-2.internal");
    // Degrade both
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::CircuitOpen);
    LoadbalancerSvc::report_outcome(&pool, &id2, Outcome::CircuitOpen);
    // Restore one
    LoadbalancerSvc::report_outcome(&pool, &id1, Outcome::Success);
    let b = LoadbalancerSvc::select(&pool).expect("restored backend must be selectable");
    assert_eq!(b.url, "https://api-1.internal");
}

#[test]
fn test_backend_pool_empty_config_returns_invalid_config_error() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = LoadbalancerSvc::build_pool(config).unwrap_err();
    assert!(
        matches!(err, swe_edge_loadbalancer::LoadbalancerError::InvalidConfig(_)),
        "empty backends must fail with InvalidConfig: {err:?}"
    );
}

#[test]
fn test_backend_pool_weighted_strategy_distributes_by_weight() {
    // Backend 1 has weight 3, backend 2 has weight 1 → backend 1 must be
    // picked in 3 of 4 consecutive selections.
    let config = LoadbalancerConfig {
        strategy: Strategy::Weighted,
        backends: vec![
            BackendConfig { url: "https://heavy.internal".to_string(), weight: 3 },
            BackendConfig { url: "https://light.internal".to_string(), weight: 1 },
        ],
    };
    let pool = LoadbalancerSvc::build_pool(config).expect("pool must build");
    let urls: Vec<String> = (0..4)
        .map(|_| LoadbalancerSvc::select(&pool).expect("must select").url)
        .collect();
    let heavy_count = urls.iter().filter(|u| u.as_str() == "https://heavy.internal").count();
    let light_count = urls.iter().filter(|u| u.as_str() == "https://light.internal").count();
    assert_eq!(heavy_count, 3, "weighted: heavy backend must get 3/4 picks");
    assert_eq!(light_count, 1, "weighted: light backend must get 1/4 picks");
}
