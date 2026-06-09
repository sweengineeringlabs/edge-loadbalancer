//! Minimal example: build a pool, select a backend, report an outcome.

use swe_edge_loadbalancer::{
    BackendConfig, LoadbalancerConfig, Outcome, Strategy,
    build_backend_pool, report_backend_outcome, select_backend,
};

fn main() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig { url: "https://api-1.internal".to_string(), weight: 1 },
            BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
        ],
    };

    let pool = build_backend_pool(config).expect("pool must build from valid config");

    for i in 0..4 {
        let backend = select_backend(&pool).expect("a healthy backend must be available");
        println!("request {i}: selected backend {}", backend.url);
        report_backend_outcome(&pool, &backend.id, Outcome::Success);
    }
}
