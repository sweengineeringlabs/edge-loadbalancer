//! Integration tests for `PoolInner` internal pool state.
//!
//! `PoolInner` is `pub(crate)` and not directly accessible from integration
//! tests. Its behaviour — strategy dispatch, round-robin counter atomicity,
//! weighted selection distribution, and least-connections ranking — is
//! exercised through `BackendPoolInstance`'s public surface.
//!
//! @covers: PoolInner

use swe_edge_loadbalancer::{
    build_backend_pool, select_backend, BackendConfig, LoadbalancerConfig, Strategy,
};

fn rr_config(n: usize) -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: (0..n)
            .map(|i| BackendConfig {
                url: format!("https://be-{i}.internal"),
                weight: 1,
            })
            .collect(),
    }
}

#[test]
fn test_pool_inner_round_robin_cycles_across_all_backends() {
    // @covers: PoolInner::rr_counter, Strategy::RoundRobin dispatch
    let pool = build_backend_pool(rr_config(3)).expect("pool must build");
    let mut urls: Vec<String> = (0..6)
        .map(|_| select_backend(&pool).expect("select must succeed").url)
        .collect();
    urls.dedup();
    // After dedup of a 2-cycle over 3 backends, we should see > 1 distinct URL,
    // confirming the counter actually advances.
    assert!(urls.len() > 1, "round-robin must distribute across backends: {urls:?}");
}

#[test]
fn test_pool_inner_round_robin_wraps_on_overflow_gracefully() {
    // @covers: PoolInner::rr_counter modular wrapping
    // 100 selects on a 2-backend pool must all succeed with no panic.
    let pool = build_backend_pool(rr_config(2)).expect("pool must build");
    for _ in 0..100 {
        assert!(select_backend(&pool).is_ok(), "every select must succeed");
    }
}

#[test]
fn test_pool_inner_weighted_strategy_respects_weight_distribution() {
    // @covers: PoolInner weighted selection branch
    let config = LoadbalancerConfig {
        strategy: Strategy::Weighted,
        backends: vec![
            BackendConfig { url: "https://heavy.internal".to_string(), weight: 9 },
            BackendConfig { url: "https://light.internal".to_string(), weight: 1 },
        ],
    };
    let pool = build_backend_pool(config).expect("pool must build");

    let heavy_count = (0..100)
        .filter(|_| {
            select_backend(&pool).map(|b| b.url.contains("heavy")).unwrap_or(false)
        })
        .count();

    // Heavy backend has 9/10 weight — expect at least 70/100 selections.
    assert!(
        heavy_count >= 70,
        "weighted strategy must prefer the heavier backend: heavy got {heavy_count}/100"
    );
}

#[test]
fn test_pool_inner_least_connections_strategy_succeeds_with_single_backend() {
    // @covers: PoolInner LeastConnections branch with single entry
    let config = LoadbalancerConfig {
        strategy: Strategy::LeastConnections,
        backends: vec![BackendConfig {
            url: "https://sole.internal".to_string(),
            weight: 1,
        }],
    };
    let pool = build_backend_pool(config).expect("pool must build");
    let result = select_backend(&pool);
    assert!(result.is_ok(), "single-backend least-connections must select successfully");
    assert_eq!(result.unwrap().url, "https://sole.internal");
}

#[test]
fn test_pool_inner_concurrent_selects_return_valid_backends() {
    // @covers: PoolInner Arc<RwLock<>> thread-safety
    use std::sync::Arc;
    use std::thread;

    let config = rr_config(4);
    let pool = Arc::new(build_backend_pool(config).expect("pool must build"));

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let p = Arc::clone(&pool);
            thread::spawn(move || select_backend(&p).is_ok())
        })
        .collect();

    for h in handles {
        assert!(h.join().expect("thread must not panic"), "concurrent select must succeed");
    }
}
