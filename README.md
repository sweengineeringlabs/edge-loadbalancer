# swe-edge-loadbalancer

Shared load balancer primitives for `swe-edge` — egress backend pools, ingress
admission, in-process autoscaling, and tenant/handler registries.

- **egress** (ADR-011) — `BackendPool`, `Strategy`, `Outcome`, `BackendHealth`, and
  supporting types for distributing outbound HTTP requests across a pool of backends.
- **ingress** (ADR-012) — `IngressLoadBalancer`, admission hints, node membership.
- **autoscale** (ADR-013) — `InstancePool`, `ScalingSignal`, `ScalingExecutor` for
  in-process concurrency gating and the runtime feedback loop.
- **registry** — `PoolRegistry`, `TenantRegistry` for `(handler, tenant)` lookup.

## Status

Six crates as of `v0.4.0`, per [`docs/adr/ADR-001`](docs/adr/ADR-001-subdomain-crate-split.md):
`-tenant`, `-egress`, `-autoscale`, `-ingress`, `-registry`, plus this crate as the
umbrella facade (`LoadbalancerSvc` — unchanged public API, delegating into the five).
A further physical port/adapter split *within* a subdomain crate is a separate,
still-open question — see [`docs/adr/ADR-002`](docs/adr/ADR-002-port-adapter.md) and
[issue #4](https://github.com/sweengineeringlabs/edge-loadbalancer/issues/4) — deferred
until a real consumer's needs justify it; as of ADR-002, nothing in the `edge`
ecosystem depends on any of these crates yet.

## Features

- **Round-robin / weighted / least-connections** strategies for egress backend
  selection.
- **Health tracking** — `report_outcome` transitions backends between `Healthy`,
  `Degraded`, and `Dead` states.
- **Concurrency-gated instance pools** with autoscale feedback (ADR-013).
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
use swe_edge_loadbalancer::{LoadbalancerConfig, LoadbalancerSvc, Outcome};

let config = LoadbalancerConfig::from_toml(include_str!("config/application.toml"))
    .expect("valid config");
let pool = LoadbalancerSvc::build_pool(config).expect("pool must build");

// Per-request
let backend = LoadbalancerSvc::select(&pool).expect("a healthy backend must be available");
// ... send the request to backend.url ...
LoadbalancerSvc::report_outcome(&pool, &backend.id, Outcome::Success);
```

## License

MIT OR Apache-2.0
