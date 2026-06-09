# swe-edge-loadbalancer

Load balancer contract for `swe-edge-egress-http`.

Provides `BackendPool`, `Strategy`, `Outcome`, `BackendHealth`, and supporting
types for distributing outbound HTTP requests across a pool of backends.

## Features

- **Round-robin** — uniform distribution across healthy backends.
- **Weighted** — proportional distribution based on per-backend weight.
- **Least-connections** — each request goes to the backend with the fewest
  in-flight connections.
- **Health tracking** — `report_outcome` transitions backends between
  `Healthy`, `Degraded`, and `Dead` states.
- **TOML config** — integrates with `swe-edge-configbuilder` via the
  `[loadbalancer]` section.

## Quick start

```toml
[loadbalancer]
strategy = "round-robin"

[[loadbalancer.backends]]
url = "https://api-1.internal"
weight = 1

[[loadbalancer.backends]]
url = "https://api-2.internal"
weight = 1
```

```rust
use swe_edge_loadbalancer::{LoadbalancerConfig, build_backend_pool, select_backend, report_backend_outcome, Outcome};

let config = LoadbalancerConfig::from_toml(include_str!("config/application.toml"))
    .expect("valid config");
let pool = build_backend_pool(config).expect("pool must build");

// Per-request
let backend = select_backend(&pool).expect("a healthy backend must be available");
// ... send the request to backend.url ...
report_backend_outcome(&pool, &backend.id, Outcome::Success);
```

## License

MIT OR Apache-2.0
