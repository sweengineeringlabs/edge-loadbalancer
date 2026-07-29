//! Minimal example: build a pool, select a backend, report an outcome.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerSvc, Outcome, Strategy,
};

fn main() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    };

    let pool = LoadbalancerSvc::build_pool(config).expect("pool must build from valid config");

    for i in 0..4 {
        let backend = LoadbalancerSvc::select(&pool).expect("a healthy backend must be available");
        println!("request {i}: selected backend {}", backend.url);
        LoadbalancerSvc::report_outcome(&pool, &backend.id, Outcome::Success);
    }
}
